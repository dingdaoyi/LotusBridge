use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use sqlx::SqlitePool;
use export_core::model::{CreateExportConfig, ExportConfig, ExportConfigListVo, ExportConfigWithPluginName};
use crate::config::db::get_conn;
use crate::models::R;
use crate::config::error::{EdgeError, Result};
use crate::models::export_config::{ExportGroup, ExportGroupQuery};

/// 添加导出配置
pub async fn create_export_config(State(pool): State<SqlitePool>,export_config: Json<CreateExportConfig>) -> Result<Json<R<ExportConfig>>> {
    let created_export_config = sqlx::query_as::<_, ExportConfig>(
        "INSERT INTO tb_export_config (name, configuration, description, plugin_id) VALUES (?, ?, ?, ?) RETURNING *",
    )
        .bind(&export_config.name)
        .bind(serde_json::to_string(&export_config.configuration).unwrap()) // Serialize the HashMap to JSON string
        .bind(&export_config.description)
        .bind(export_config.plugin_id)
        .fetch_one(&pool)
        .await
        .map_err(|err| EdgeError::Message(err.to_string()))?;

    Ok(Json(R::success_with_data(created_export_config)))
}

/// 关联设备群组
pub async fn associated_device_group(Json(export_group): Json<ExportGroupQuery>) -> Result<Json<R<String>>> {
    // 删除之前关联数据
    delete_export_group(&export_group.export_id).await?;
    // 添加新的关联数据
    let query = format!(
        "INSERT INTO tb_export_group (export_id, group_id) VALUES {}",
        export_group
            .group_ids
            .iter()
            .map(|_| "(?, ?)")
            .collect::<Vec<_>>()
            .join(", ")
    );
    let mut query_builder = sqlx::query(&query);

    for group_id in &export_group.group_ids {
        query_builder = query_builder.bind(export_group.export_id).bind(group_id);
    }

    let affected_rows=  query_builder.execute(&get_conn()).await?;

    // 更新导出插件
    if affected_rows.rows_affected() > 0 {
        // 更新导出插件
        Ok(Json(R::success()))
    } else {
        // 添加失败
        Ok(Json(R::fail("关联失败".into())))
    }
}

/// 查询关联的设备群组
pub async fn list_export_group(Path(export_id): Path<i32>) -> Result<Json<R<Vec<ExportGroup>>>> {
   let  res= sqlx::query_as::<_,ExportGroup>("select * from tb_export_group where export_id=?")
        .bind(export_id)
        .fetch_all(&get_conn()).await?;
    Ok(Json(R::success_with_data(res)))
}

/// 获取详情
pub async fn get_export_config(State(pool): State<SqlitePool>, Path(id): Path<i32>) -> Result<Json<R<ExportConfig>>> {
    let export_config = sqlx::query_as::<_, ExportConfig>("SELECT * FROM tb_export_config WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?;

    match export_config {
        Some(export_config) => Ok(Json(R::success_with_data(export_config))),
        None => Err(EdgeError::Message("导出配置不存在".into())),
    }
}

#[derive(Deserialize)]
pub struct ExportConfigQuery {
    #[serde(rename = "pluginId")]
    plugin_id: Option<i32>,
}


/// 导出列表
pub async fn list_export_config(Query(ExportConfigQuery { plugin_id, .. }): Query<ExportConfigQuery>) -> Result<Json<R<Vec<ExportConfigListVo>>>> {

    let mut query_str= r#"
    SELECT ec.*, GROUP_CONCAT(dg.name, ',') AS group_names
         FROM tb_export_config ec
         LEFT JOIN tb_export_group eg ON ec.id = eg.export_id
         LEFT JOIN tb_device_group dg ON eg.group_id = dg.id
    "#.to_string();
    let pool = get_conn();

    if let Some(plugin_id_value) = plugin_id {
        query_str.push_str(" WHERE ec.plugin_id = ? GROUP BY ec.id");
        let export_config_list = sqlx::query_as::<_, ExportConfigListVo>(&query_str)
            .bind(plugin_id_value)
            .fetch_all(&pool)
            .await?;
        Ok(Json(R::success_with_data(export_config_list)))
    } else {
        query_str.push_str("  GROUP BY ec.id");
        let export_config_list = sqlx::query_as::<_, ExportConfigListVo>(&query_str)
            .fetch_all(&pool)
            .await?;
        Ok(Json(R::success_with_data(export_config_list)))
    }
}


/// 导出列表
pub async fn load_all_export_config() -> Result<Vec<ExportConfigWithPluginName>> {
    let query_str = r#"
        SELECT
            tb_export_config.id,
            tb_export_config.name,
            tb_export_config.configuration,
            tb_export_config.description,
            tb_export_config.plugin_id,
            plugin_config.name AS plugin_name
        FROM
            tb_export_config
        JOIN
            plugin_config ON tb_export_config.plugin_id = plugin_config.id
    "#;

    let pool = get_conn();
    let export_config_list = sqlx::query_as::<_, ExportConfigWithPluginName>(query_str)
        .fetch_all(&pool)
        .await?;
    Ok(export_config_list)
}


/// 修改
pub async fn update_export_config(State(pool): State<SqlitePool>, Path(id): Path<i32>, export_config: Json<ExportConfig>) -> Result<Json<R<String>>> {
    let updated_export_config = sqlx::query(
        "UPDATE tb_export_config SET name = $1, configuration = $2, description = $3, plugin_id = $4 WHERE id = $5",
    )
        .bind(&export_config.name)
        .bind(serde_json::to_string(&export_config.configuration).unwrap()) // Serialize the HashMap to JSON string
        .bind(&export_config.description)
        .bind(export_config.plugin_id)
        .bind(id)
        .execute(&pool)
        .await?;

    if updated_export_config.rows_affected() > 0 {
        Ok(Json(R::success()))
    } else {
        Err(EdgeError::Message("导出配置不存在".into()))
    }
}
/// 删除
pub async fn delete_export_config(State(pool): State<SqlitePool>, Path(id): Path<i32>) -> Result<Json<R<String>>> {
    let deleted_export_config = sqlx::query(
        "DELETE FROM tb_export_config WHERE id = $1",
    )
        .bind(id)
        .execute(&pool)
        .await?;

    if deleted_export_config.rows_affected() > 0 {
        Ok(Json(R::success()))
    } else {
        Err(EdgeError::Message("导出配置不存在".into()))
    }
}

/// 删除导出服务下的设备
pub async fn delete_export_group(export_id: &i32) -> Result<bool> {
    let _ = sqlx::query(
        "DELETE FROM tb_export_group WHERE export_id = $1",
    )
        .bind(export_id)
        .execute(&get_conn())
        .await?;
    Ok(true)
}
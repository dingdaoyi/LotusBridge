use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use sqlx::SqlitePool;
use export_core::model::{CreateExportConfig, ExportConfig};
use crate::config::db::get_conn;
use crate::models::R;
use crate::config::error::{EdgeError, Result};

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
pub async fn list_export_config(Query(ExportConfigQuery { plugin_id, .. }): Query<ExportConfigQuery>) -> Result<Json<R<Vec<ExportConfig>>>> {
    let mut query_str = "SELECT * FROM tb_export_config".to_string();
    let pool = get_conn();

    if let Some(plugin_id_value) = plugin_id {
        query_str.push_str(" WHERE plugin_id = ?");
        let export_config_list = sqlx::query_as::<_, ExportConfig>(&query_str)
            .bind(plugin_id_value)
            .fetch_all(&pool)
            .await?;
        Ok(Json(R::success_with_data(export_config_list)))
    } else {
        let export_config_list = sqlx::query_as::<_, ExportConfig>(&query_str)
            .fetch_all(&pool)
            .await?;

        Ok(Json(R::success_with_data(export_config_list)))
    }
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

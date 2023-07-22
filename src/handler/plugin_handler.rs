use axum::extract::{Path, Query, State};
use axum::Json;
use sqlx::{Connection, Executor, SqlitePool};
use crate::config::error::{EdgeError, Result};
use crate::models::plugin::{CreatePluginConfig, PluginConfig, PluginConfigQuery};
use crate::models::R;

pub async fn create_plugin_config(
    State(pool): State<SqlitePool>,
    Json(plugin_config): Json<CreatePluginConfig>,
) -> Result<Json<R<String>>> {
    let mut conn = pool.acquire().await?;
    let mut transaction = conn.begin().await?;
    transaction
        .execute(
            sqlx::query(
                "INSERT INTO plugin_config ( name , description, form_customization, plugin_type) VALUES (?, ?, ?, ?)",
            )
                .bind(&plugin_config.description)
                .bind(&plugin_config.name)
                .bind(&plugin_config.form_customization)
                .bind(&plugin_config.plugin_type),
        )
        .await?;
    transaction.commit().await?;
    Ok(Json(R::success()))
}


pub async fn update_plugin_config(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
    Json(plugin_config): Json<PluginConfig>,
) -> Result<Json<R<String>>> {
    let updated_plugin_config = sqlx::query(
        "UPDATE plugin_config SET name = $1, description = $2, form_customization = $3, plugin_type = $4 WHERE id = $5",
    )
        .bind(&plugin_config.name)
        .bind(&plugin_config.description)
        .bind(sqlx::types::Json(&plugin_config.form_customization))
        .bind(plugin_config.plugin_type)
        .bind(id)
        .execute(&pool)
        .await?;

    if updated_plugin_config.rows_affected() > 0 {
        Ok(Json(R::success_with_data("更新成功".into())))
    } else {
        Err(EdgeError::Message("插件不存在".into()))
    }
}


pub async fn list_plugin(
    State(pool): State<SqlitePool>,
    Query(PluginConfigQuery { name, plugin_type, .. }): Query<PluginConfigQuery>,
) -> Result<Json<R<Vec<PluginConfig>>>> {
    let mut sql = format!("SELECT * FROM plugin_config WHERE 1 = 1");

    if let Some(_) = &name {
        sql.push_str(" AND name = ?");
    }

    if let Some(_) = &plugin_type {
        sql.push_str(" AND plugin_type = ?");
    }

    let plugin_configs: Vec<PluginConfig> = sqlx::query_as(&sql)
        .bind(&name)
        .bind(&plugin_type)
        .fetch_all(&pool)
        .await?;
    Ok(Json(R::success_with_data(plugin_configs)))
}


pub async fn plugin_config_details(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<Json<R<PluginConfig>>> {
    let plugin_config: Option<PluginConfig> = sqlx::query_as(
        "select * from plugin_config where id =?",
    )
        .bind(id)
        .fetch_optional(&pool)
        .await?;
    let plugin_config = plugin_config.ok_or(EdgeError::Message("插件不存在".to_string()))?;
    Ok(Json(R::success_with_data(plugin_config)))
}

pub async fn delete_plugin_config(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<Json<R<String>>> {
    let res = sqlx::query(
        "delete from plugin_config where id =?",
    )
        .bind(id)
        .execute(&pool)
        .await?;
    if res.rows_affected() > 0 {
        Ok(Json(R::success_with_data("删除成功".into())))
    } else {
        Err(EdgeError::Message("删除失败".into()))
    }
}
use axum::extract::State;
use axum::Json;
use sqlx::{Connection, Executor, Sqlite, SqlitePool};
use crate::config::error::{Result};
use crate::models::plugin::CreatePluginConfig;
use crate::models::R;
use crate::utils::generate_unique_id;

pub async fn create_plugin_config(
    State(pool): State<SqlitePool>,
    Json(plugin_config): Json<CreatePluginConfig>,
) -> Result<Json<R<String>>> {
    let mut conn = pool.acquire().await?;
    let mut transaction = conn.begin().await?;
    let plugin_config_id = generate_unique_id();
    insert_plugin_config(&mut transaction, &plugin_config, plugin_config_id).await?;
    transaction.commit().await?;
    Ok(Json(R::success()))
}

async fn insert_plugin_config(
    transaction: &mut sqlx::Transaction<'_, Sqlite>,
    plugin_config: &CreatePluginConfig,
    plugin_config_id: i64,
) -> Result<()> {
    transaction
        .execute(
            sqlx::query(
                "INSERT INTO plugin_config (id, name , description, form_customization, plugin_type) VALUES (?,?, ?, ?, ?)",
            )
                .bind(plugin_config_id)
                .bind(&plugin_config.description)
                .bind(&plugin_config.name)
                .bind(&plugin_config.form_customization)
                .bind(&plugin_config.plugin_type),
        )
        .await?;

    Ok(())
}
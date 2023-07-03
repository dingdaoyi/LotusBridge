use axum::extract::State;
use axum::Json;
use sqlx::{Connection, Executor, SqlitePool};
use validator::Validate;
use crate::config::error::{Result};
use crate::models::plugin::{CreatePluginConfig, CreatePlugin, CreateProtocolConfig};
use crate::models::R;
use crate::utils::generate_unique_id;

pub async fn create_plugin_config(
    State(pool): State<SqlitePool>,
    Json(plugin_config): Json<CreatePluginConfig>,
) -> Result<Json<R<String>>> {
    let mut conn = pool.acquire().await?;
    let mut transaction = conn.begin().await?;
    let plugin_config_id = generate_unique_id();
    insert_plugin_config(&mut transaction, &plugin_config, plugin_config_id.clone()).await?;
    insert_plugin(&mut transaction, &plugin_config.plugin, plugin_config_id).await?;
    transaction.commit().await?;
    Ok(Json(R::success()))
}

async fn insert_plugin_config(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    plugin_config: &CreatePluginConfig,
    plugin_config_id: i64,
) -> Result<()> {
    transaction
        .execute(
            sqlx::query(
                "INSERT INTO plugin_config (id, description, form_customization, plugin_type) VALUES (?, ?, ?, ?)",
            )
                .bind(plugin_config_id)
                .bind(&plugin_config.description)
                .bind(&plugin_config.form_customization)
                .bind(&plugin_config.plugin_type),
        )
        .await?;

    Ok(())
}

async fn insert_plugin(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    plugin: &CreatePlugin,
    plugin_config_id: i64,
) -> Result<()> {
    match plugin {
        CreatePlugin::Protocol(protocol_config) => {
            //TODO  暂时测试,没有找到集成到axum的方式,测试错误,先睡觉
            protocol_config.validate()?;
            insert_protocol_config(transaction, protocol_config, plugin_config_id).await?;
        }
        CreatePlugin::DataOutput(_data_output_config) => {
            // 执行 data_output_config 的插入操作
        }
        CreatePlugin::RuleEngine(_rule_engine_config) => {
            // 执行 rule_engine_config 的插入操作
        }
    }

    Ok(())
}

async fn insert_protocol_config(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    protocol_config: &CreateProtocolConfig,
    plugin_config_id: i64,
) -> Result<()> {
    transaction
        .execute(
            sqlx::query(
                "INSERT INTO protocol_config (name, path, description, plugin_config_id) VALUES (?, ?, ?, ?)",
            )
                .bind(&protocol_config.name)
                .bind(&protocol_config.path)
                .bind(&protocol_config.description)
                .bind(plugin_config_id),
        )
        .await?;
    Ok(())
}

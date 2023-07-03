use axum::extract::{Path, State};
use axum::{Extension, Json};
use sqlx::SqlitePool;
use crate::config::error::{Result};
use crate::config::cache::ProtocolStore;
use crate::models::plugin::ProtocolConfig;
use crate::models::R;

pub async fn load_protocol(State(pool): State<SqlitePool>,
                           Extension(protocol_store): Extension<ProtocolStore>,
                           Path(id): Path<i64>) -> Result<Json<R<String>>> {
    // 加载插件
    let protocol_config = sqlx::query_as::<_, ProtocolConfig>("SELECT * FROM protocol_config WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?;

    // 判断内存 protocol_store 中是否已加载
    if let Some(config) = protocol_config {
        let name = config.name.clone();
        if let Some(_) = protocol_store.get_protocol(&name)? {
            return Ok(Json(R::success()));
        }

        // 加载到内存中
        protocol_store.load_protocol(&config)?;
    }
    //返回数据
    Ok(Json(R::success()))
}

use axum::{Extension, Router};
use axum::routing::{delete, get, post, put};
use crate::handler::things::{get_product_by_id, get_product_funcs};
use sqlx::{SqlitePool};
use tower::{ServiceBuilder};
use crate::handler::plugin_handler::create_plugin_config;
use crate::handler::device_handler::{create_device, delete_device, get_device, update_device};

use crate::config::cache::ProtocolStore;
use crate::handler::protocol::load_protocol;

pub fn register(pool: SqlitePool) -> Router {
    Router::new()
        .route("/things", get(get_product_funcs))
        .route("/things/:id", get(get_product_by_id))
        // 设备
        .route("/device/:id", get(get_device))
        .route("/device", post(create_device))
        .route("/device/:id", put(update_device))
        .route("/device/:id", delete(delete_device))
        // 协议处理
        .route("/load/protocol/:id", get(load_protocol))
        //创建插件
        .route("/plugin", post(create_plugin_config)
        )
       .with_state(pool)
        .layer(
                ServiceBuilder::new()
                    .layer(Extension(ProtocolStore::new()))
                    .into_inner(),
        )
        // .with_state(cache::ProtocolStore::new())
}
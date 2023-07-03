use axum::{Extension, Router};
use axum::routing::{delete, get, post, put};
use crate::handler::things::{get_product_by_id, get_product_funcs};
use sqlx::{SqlitePool};
use tower::{ServiceBuilder};
use crate::handler::plugin::create_plugin_config;
use crate::handler::product::{create_product, delete_product, get_product, update_product};

use crate::config::cache::ProtocolStore;
use crate::handler::protocol::load_protocol;

pub fn register(pool: SqlitePool) -> Router {
    Router::new()
        .route("/things", get(get_product_funcs))
        .route("/things/:id", get(get_product_by_id))
        // 产品
        .route("/product/:id", get(get_product))
        .route("/product", get(create_product))
        .route("/product/:id", put(update_product))
        .route("/product/:id", delete(delete_product))
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
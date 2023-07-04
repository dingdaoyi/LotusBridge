use axum::{Extension, Router};
use axum::middleware::{from_extractor};
use axum::routing::{delete, get, post, put};
use crate::handler::things::{get_product_by_id, get_product_funcs};
use sqlx::{SqlitePool};
use crate::handler::plugin_handler::create_plugin_config;
use crate::handler::device_handler::{create_device, delete_device, get_device, update_device};
use crate::config::cache::ProtocolStore;
use crate::handler::auth_handler;
use crate::handler::auth_handler::login;
use crate::handler::point_handler::{create_point, delete_point, get_point, update_point};
use crate::handler::protocol::load_protocol;

pub fn register(pool: SqlitePool) -> Router {
    Router::new()
        .nest("/", routers())
        .with_state(pool)
        .layer(Extension(ProtocolStore::new()))
}


//api
pub fn routers() -> Router<SqlitePool> {
    need_auth_routers().merge(no_need_auth_routers())
}

//需要权限认证的路由
pub fn need_auth_routers() -> Router<SqlitePool> {
    Router::new()
        .route("/things", get(get_product_funcs))
        .route("/things/:id", get(get_product_by_id))
        // 设备
        .route("/device/:id", get(get_device))
        .route("/device", post(create_device))
        .route("/device/:id", put(update_device))
        .route("/device/:id", delete(delete_device))
        //端点
        .route("/point/:id", get(get_point))
        .route("/point", post(create_point))
        .route("/point/:id", put(update_point))
        .route("/point/:id", delete(delete_point))
        // 协议处理
        .route("/load/protocol/:id", get(load_protocol))
        //创建插件
        .route("/plugin", post(create_plugin_config),
        )
        .layer(from_extractor::<auth_handler::Claims>())
}

//不需要权限认证的路由
pub fn no_need_auth_routers() -> Router<SqlitePool> {
    Router::new()
        .route("/login", post(login))
}
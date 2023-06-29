use axum::Router;
use axum::routing::{delete, get, put};
use crate::handler::things::{get_product_by_id, get_product_funcs};
use sqlx::{SqlitePool};
use crate::handler::product::{create_product, delete_product, get_product, update_product};
use crate::handler::protocol::test_protocol;

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
        .route("/protocol", get(test_protocol))

        .with_state(pool)
}
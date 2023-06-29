use axum::Router;
use axum::routing::get;
use crate::controller::things::{get_product_by_id, get_product_funcs};
use sqlx::{SqlitePool};

pub fn register(pool: SqlitePool) -> Router {
    Router::new()
        .route("/things", get(get_product_funcs))
        .route("/things/:id", get(get_product_by_id))
        .with_state(pool)
}
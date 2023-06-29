use crate::models::things::ProductFunc;
use axum::extract::{Path, Query, State};
use axum::{Json};
use serde::Deserialize;
use sqlx::SqlitePool;
use crate::config::error::EdgeError;
use crate::service::thing_service::ProductFuncService;


#[derive(Deserialize)]
pub struct PaginationParams {
    page: Option<usize>,
    limit: Option<usize>,
}


pub async fn get_product_funcs(
    State(db_pool): State<SqlitePool>,
    pagination: Query<PaginationParams>,
) -> Result<Json<Vec<ProductFunc>>, EdgeError> {
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);
    let offset = (page - 1) * limit;
    let vector = sqlx::query_as::<_, ProductFunc>(
        r#"
        SELECT * FROM product_func
        LIMIT $1 OFFSET $2
        "#,
    )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&db_pool)
        .await?;
    Ok(Json(vector))
}

pub async fn get_product_by_id(
    State(db_pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<ProductFunc>, EdgeError> {
    ProductFuncService::new(db_pool)
        .get_thing(id).await
        .map(|prod_func| Json(prod_func))
}

use crate::config::error::Result;
use crate::models::things::ProductFunc;
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize)]
pub struct PaginationParams {
    page: Option<usize>,
    limit: Option<usize>,
}

pub async fn get_product_funcs(
    State(db_pool): State<SqlitePool>,
    pagination: Query<PaginationParams>,
) -> Result<Json<Vec<ProductFunc>>> {
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
) -> Result<Json<ProductFunc>> {
    let product_func =
        sqlx::query_as::<_, ProductFunc>("SELECT * FROM product_func  where id =$1 LIMIT 1")
            .bind(id)
            .fetch_one(&db_pool)
            .await?;
    Ok(Json(product_func))
}

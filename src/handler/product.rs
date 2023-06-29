use axum::extract::{Path, State};
use axum::Json;
use sqlx::SqlitePool;
use crate::config::error::{Result};
use crate::models::product::{CreatProduct, Product};
use crate::utils::id_util;

pub async fn get_product(State(pool): State<SqlitePool>, Path(product_id): Path<i64>) -> Result<Json<Product>> {
    let product = sqlx::query_as::<_, Product>("SELECT * FROM tb_product WHERE id = ?")
        .bind(product_id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(product))
}

pub async fn create_product(State(pool): State<SqlitePool>, product: Json<CreatProduct>) -> Result<Json<Product>> {
    let created_product = sqlx::query_as::<_, Product>(
        "INSERT INTO tb_product (id, name, product_type) VALUES (?, ?, ?) RETURNING *",
    )
        .bind(id_util::generate_unique_id())
        .bind(&product.name)
        .bind(&product.product_type)
        .fetch_one(&pool)
        .await?;

    Ok(Json(created_product))
}

pub async fn update_product(
    State(pool): State<SqlitePool>,
    Path(product_id): Path<i64>,
    product: Json<Product>,
) -> Result<Json<Product>> {
    let updated_product = sqlx::query_as::<_, Product>(
        "UPDATE tb_product SET name = ?, product_type = ? WHERE id = ? RETURNING *",
    )
        .bind(&product.name)
        .bind(&product.product_type)
        .bind(product_id as i64)
        .fetch_one(&pool)
        .await?;

    Ok(Json(updated_product))
}

pub async fn delete_product(State(pool): State<SqlitePool>, Path(product_id): Path<i64>) -> Result<Json<()>> {
    sqlx::query("DELETE FROM tb_product WHERE id = ?")
        .bind(product_id)
        .execute(&pool)
        .await?;
    Ok(Json(()))
}
use axum::extract::{Path, State};
use axum::Json;
use sqlx::SqlitePool;
use protocol_core::Point;
use crate::config::error::{EdgeError, Result};
use crate::models::point::CreatePoint;
use crate::models::R;

pub async fn get_point(State(pool): State<SqlitePool>, Path(id): Path<i32>) -> Result<Json<Point>> {
    let point = sqlx::query_as::<_, Point>("SELECT * FROM tb_point WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?;
    match point {
        Some(point) => Ok(Json(point)),
        None => {
            // 没有找到匹配的行，返回自定义错误或其他逻辑
            Err(EdgeError::Message("未找到指定的数据行".into()))
        }
    }
}

pub async fn create_point(State(pool): State<SqlitePool>,Json(point): Json<CreatePoint>) -> Result<Json<R<Point>>> {
    let created_point = sqlx::query_as::<_, Point>(
        "INSERT INTO tb_point (device_id, address, data_type, access_mode, multiplier, precision, description, part_number) VALUES (?, ?, ?, ?, ?, ?, ?, ?) RETURNING *",
    )
        .bind(point.device_id)
        .bind(&point.address)
        .bind(&point.data_type)
        .bind(&point.access_mode)
        .bind(point.multiplier)
        .bind(point.precision)
        .bind(&point.description)
        .bind(&point.part_number)
        .fetch_one(&pool)
        .await?;

    Ok(Json(R::success_with_data(created_point)))
}

pub async fn update_point(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
    Json(point): Json<Point>,
) -> Result<Json<R<String>>> {
    let updated_point = sqlx::query(
        "UPDATE tb_point SET address = $1, data_type = $2, access_mode = $3, multiplier = $4, precision = $5, description = $6, part_number = $7, device_id = $8  WHERE id = $9",
    )
        .bind(&point.address)
        .bind(&point.data_type)
        .bind(&point.access_mode)
        .bind(point.multiplier)
        .bind(point.precision)
        .bind(&point.description)
        .bind(&point.part_number)
        .bind(point.device_id)
        .bind(id)
        .execute(&pool)
        .await?;

    // 检查是否成功更新了点
    if updated_point.rows_affected() > 0 {
        // 返回更新后的点
        Ok(Json(R::success()))
    } else {
        // 如果没有更新点，则返回错误信息
        Err(EdgeError::Message("点位不存在".into()))
    }
}

pub async fn delete_point(State(pool): State<SqlitePool>, Path(point_id): Path<i32>) -> Result<Json<R<String>>> {
    sqlx::query("DELETE FROM tb_point WHERE id = ?")
        .bind(point_id)
        .execute(&pool)
        .await?;

    Ok(Json(R::success()))
}

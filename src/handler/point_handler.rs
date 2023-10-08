use crate::config::cache::get_protocol_store;
use crate::config::db::get_conn;
use crate::config::device_shadow;
use crate::config::error::{EdgeError, Result};
use crate::models::device::DeviceGroupWithExportName;
use crate::models::page::PaginationResponse;
use crate::models::point::{CreatePoint, PointPageQuery};
use crate::models::R;
use axum::extract::{Path, State};
use axum::Json;
use export_core::model::{DeviceGroupValue, PointValue};
use futures::future::join_all;
use futures::TryFutureExt;
use protocol_core::{Point, PointWithProtocolId, Value, WriterPointRequest};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::handler::device_handler::list_all_device_group;

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

pub async fn point_page(
    State(pool): State<SqlitePool>,
    Json(point_query): Json<PointPageQuery>,
) -> Result<Json<R<PaginationResponse<Point>>>> {
    let request = point_query.page;
    let offset = (request.page - 1) * request.limit;
    let limit = request.limit;
    return match point_query.name {
        None => {
            let count_query =
                sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM tb_point WHERE group_id = ? ")
                    .bind(point_query.group_id);
            let total_count = count_query.fetch_one(&pool).await?;
            let mut points = vec![];
            if total_count > 0 {
                points = sqlx::query_as::<_, Point>(
                    "SELECT * FROM tb_point WHERE group_id = ?  LIMIT ? OFFSET ?",
                )
                .bind(point_query.group_id)
                .bind(limit)
                .bind(offset)
                .fetch_all(&pool)
                .await?;
            }
            Ok(Json(R::success_with_data(PaginationResponse::new(
                points,
                total_count as u32,
            ))))
        }
        Some(name) => {
            let count_query = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM tb_point WHERE group_id = ? AND name like '%?%'",
            )
            .bind(point_query.group_id)
            .bind(&name);
            let total_count = count_query.fetch_one(&pool).await?;
            let mut points = vec![];
            if total_count > 0 {
                points = sqlx::query_as::<_, Point>("SELECT * FROM tb_point WHERE group_id = ?  AND name like '%?%'  LIMIT ? OFFSET ? ")
                    .bind(point_query.group_id)
                    .bind(&name)
                    .bind(limit)
                    .bind(offset)
                    .fetch_all(&pool)
                    .await?;
            }
            Ok(Json(R::success_with_data(PaginationResponse::new(
                points,
                total_count as u32,
            ))))
        }
    };
}

pub async fn exists_by_group_id(group_id: i32) -> bool {
    return sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM tb_point WHERE group_id = ?)")
        .bind(group_id)
        .fetch_one(&get_conn())
        .await
        .unwrap_or(false);
}

pub async fn create_point(
    State(pool): State<SqlitePool>,
    Json(point): Json<CreatePoint>,
) -> Result<Json<R<Point>>> {
    let device_id =
        sqlx::query_scalar::<_, i32>("select device_id  from tb_device_group where id =?")
            .bind(point.group_id)
            .fetch_optional(&pool)
            .await?;
    let device_id = device_id.ok_or(EdgeError::Message("设备组不存在,请检查参数!".into()))?;
    let exists = sqlx::query("SELECT 1 FROM tb_point WHERE device_id = ? AND address = ?")
        .bind(&device_id)
        .bind(&point.address)
        .fetch_optional(&pool)
        .await?;

    if exists.is_some() {
        return Err(EdgeError::Message(
            "当前设备下点位已存在,请勿重复添加!".into(),
        ));
    }

    let created_point = sqlx::query_as::<_, Point>(
        "INSERT INTO tb_point (device_id,group_id, address, data_type, access_mode, multiplier, precision, description, part_number) VALUES (?, ?, ?, ?, ?, ?, ?, ?,?) RETURNING *",
    )
        .bind(device_id)
        .bind(point.group_id)
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

pub async fn delete_point(
    State(pool): State<SqlitePool>,
    Path(point_id): Path<i32>,
) -> Result<Json<R<String>>> {
    sqlx::query("DELETE FROM tb_point WHERE id = ?")
        .bind(point_id)
        .execute(&pool)
        .await?;

    Ok(Json(R::success()))
}

pub async fn read_point_value(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<Json<R<Value>>> {
    let point = get_point_with_protocol_id(pool, id).await?;
    let res = device_shadow::read_point(point.protocol_name.clone(), point.into())
        .await
        .map(|e| e.value)?;
    Ok(Json(R::success_with_data(res)))
}


///此处需要优化,目前是获取所有group_id,后续需要优化为根据group_id查询
pub async fn read_group_point_values(
    State(pool): State<SqlitePool>,
    Path(group_id): Path<i32>,
) -> Result<Json<R<Vec<PointValue>>>> {
    let device_group_list=list_all_device_group(pool.clone()).await?;
   let device_group= device_group_list.into_iter().find(|device_group|device_group.id==group_id).ok_or(EdgeError::Message("设备组不存在,请检查参数!".into()))?;
   let device_group_value= read_point_group_value(device_group).await?;
    Ok(Json(R::success_with_data(device_group_value.point_values)))
}


pub async fn read_point_group_value(
    device_group: DeviceGroupWithExportName,
) -> Result<DeviceGroupValue> {
    let point_list = get_points_with_group_id(get_conn(), device_group.id).await?;

    // 使用异步并发并行读取每个点的值
    let tasks = point_list.iter().map(|point| {
        let protocol_name = point.protocol_name.clone();
        let point_clone = point.clone();
        let value = device_shadow::read_point(protocol_name, point_clone.clone().into());
        value.and_then(|e| async move {
            let mut point_value: PointValue = point_clone.into();
            point_value.value = Some(e.value);
            Ok(point_value)
        })
    });
    let results = join_all(tasks).await;
    for result in &results {
        if let Err(err) = result {
            return Err(EdgeError::Message(err.to_string()));
        }
    }
    let value_list: Vec<PointValue> = results.into_iter().map(|res| res.unwrap()).collect();
    let mut res: DeviceGroupValue = device_group.into();
    res.point_values = value_list;
    Ok(res)
}

#[derive(Debug, Deserialize)]
pub struct WriterValue {
    value: Value,
}

//TODO 这儿只有阻塞才不报错
pub async fn writer_point_value(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
    Json(WriterValue { value, .. }): Json<WriterValue>,
) -> Result<Json<R<Value>>> {
    let point = get_point_with_protocol_id(pool, id).await?;
    let store = get_protocol_store().unwrap();
    let protocol_map = store.inner.lock().await;
    let protocol = protocol_map
        .get(&point.protocol_name)
        .ok_or(EdgeError::Message("协议不存在,检查服务配置".into()))?;
    let mut request: WriterPointRequest = point.into();
    request.value = value;
    let res = protocol.lock().await.write_point(request).await?;
    Ok(Json(R::success_with_data(res)))
}

async fn get_point_with_protocol_id(pool: SqlitePool, id: i32) -> Result<PointWithProtocolId> {
    let point = sqlx::query_as::<_, PointWithProtocolId>(r#"
    SELECT tb_point.id AS point_id, tb_point.device_id, tb_point.group_id, tb_point.address, tb_point.data_type, tb_point.access_mode,
       tb_point.multiplier, tb_point.precision, tb_point.description, tb_point.part_number, tb_device.protocol_name AS protocol_name
        FROM tb_point
        JOIN tb_device ON tb_point.device_id = tb_device.id
        WHERE tb_point.id = ?;
    "#)
        .bind(id)
        .fetch_optional(&pool)
        .await?;
    let point = match point {
        Some(point) => point,
        None => {
            return Err(EdgeError::Message("point不存在,请检查请求参数".into()));
        }
    };
    Ok(point)
}

///根据设备group_id查询
async fn get_points_with_group_id(
    pool: SqlitePool,
    group_id: i32,
) -> Result<Vec<PointWithProtocolId>> {
    let point_list = sqlx::query_as::<_, PointWithProtocolId>(r#"
        SELECT tb_point.id AS point_id, tb_point.device_id,tb_point.group_id, tb_point.address, tb_point.data_type, tb_point.access_mode,
               tb_point.multiplier, tb_point.precision, tb_point.description, tb_point.part_number, tb_device.protocol_name AS protocol_name
        FROM tb_point
        JOIN tb_device ON tb_point.device_id = tb_device.id
        WHERE tb_point.group_id = ?;
    "#)
        .bind(group_id)
        .fetch_all(&pool)
        .await?;
    Ok(point_list)
}

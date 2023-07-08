use std::collections::HashMap;
use axum::extract::{Path, State};
use axum::Json;
use sqlx::{Row, SqlitePool};
use protocol_core::{AccessMode, DataType, Device, DeviceType, Point, PointWithProtocolId, Value};
use crate::config::error::{EdgeError, Result};
use crate::models::device::{CreatDevice, DeviceDTO};
use crate::models::R;
use crate::config::device_shadow;
use sqlx::FromRow;

pub async fn get_device(State(pool): State<SqlitePool>, Path(id): Path<i32>) -> Result<Json<DeviceDTO>> {
    let device = sqlx::query_as::<_, DeviceDTO>("SELECT * FROM tb_device WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?;
    match device {
        Some(device) => Ok(Json(device)),
        None => {
            // 没有找到匹配的行，返回自定义错误或其他逻辑
            Err(EdgeError::Message("设备不存在".into()))
        }
    }
}

pub async fn get_device_details(State(pool): State<SqlitePool>, Path(id): Path<i32>) -> Result<Json<Device>> {
    let device = sqlx::query_as::<_, DeviceDTO>("SELECT * FROM tb_device WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    let points = sqlx::query_as::<_, Point>("SELECT * FROM tb_point WHERE device_id = ?")
        .bind(device.id)
        .fetch_all(&pool)
        .await?;

    let device_with_points = Device {
        id: device.id,
        name: device.name,
        device_type: device.device_type,
        points,
        custom_data: device.custom_data.0,
        protocol_id: device.protocol_id,
    };

    Ok(Json(device_with_points))
}

pub async fn read_point_value(State(pool): State<SqlitePool>, Path(id): Path<i32>) -> Result<Json<Value>> {

    let point = sqlx::query_as::<_, PointWithProtocolId>(r#"
    SELECT tb_point.id AS point_id, tb_point.device_id, tb_point.address, tb_point.data_type, tb_point.access_mode,
       tb_point.multiplier, tb_point.precision, tb_point.description, tb_point.part_number, tb_device.protocol_id AS protocol_id
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
        },
    };
   let res= device_shadow::read_point(point.protocol_id,point.into())
       .map(|e|e.value).unwrap_or(Value::Boolean(false));
    Ok(Json(res))
}


pub async fn load_all_device_details(pool: SqlitePool) -> Result<HashMap<i32, Vec<Device>>> {
    let device_list = sqlx::query_as::<_, DeviceDTO>("SELECT * FROM tb_device")
        .fetch_all(&pool)
        .await?;
    let mut res:HashMap<i32,Vec<Device>>=HashMap::new();
    for device in device_list.iter() {
        let points = sqlx::query_as::<_, Point>("SELECT * FROM tb_point WHERE device_id = ?")
            .bind(device.id)
            .fetch_all(&pool)
            .await?;
        let device_with_points = Device {
            id: device.id,
            name: device.name.clone(),
            device_type: device.device_type.clone(),
            points,
            custom_data: device.custom_data.0.clone(),
            protocol_id: device.protocol_id,
        };
        // 插入方式简洁处理
        res.entry(device.protocol_id)
            .or_insert_with(Vec::new)
            .push(device_with_points);
    }
    tracing::info!("加载协议总数:{}",res.len());
    Ok(res)
}

pub async fn create_device(State(pool): State<SqlitePool>, device: Json<CreatDevice>) -> Result<Json<R<DeviceDTO>>> {
    let created_device = sqlx::query_as::<_, DeviceDTO>(
        "INSERT INTO tb_device (name, device_type, custom_data, protocol_id) VALUES (?, ?, ?, ?) RETURNING *",
    )
        .bind(&device.name)
        .bind(&device.device_type)
        .bind(sqlx::types::Json(&device.custom_data))
        .bind(device.protocol_id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(R::success_with_data(created_device)))
}

pub async fn update_device(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
    Json(device): Json<DeviceDTO>,
) -> Result<Json<DeviceDTO>> {
    let updated_device = sqlx::query(
        "UPDATE tb_device SET name = $1, device_type = $2, custom_data = $3, protocol_id = $4 WHERE id = $5",
    )
        .bind(&device.name)
        .bind(&device.device_type)
        .bind(sqlx::types::Json(&device.custom_data))
        .bind(device.protocol_id)
        .bind(id)
        .execute(&pool)
        .await?;

    // 检查是否成功更新了设备
    if updated_device.rows_affected() > 0 {
        // 返回更新后的设备
        Ok(Json(device))
    } else {
        // 如果没有更新设备，则返回错误信息
        Err(EdgeError::Message("设备不存在".into()))
    }
}

pub async fn delete_device(State(pool): State<SqlitePool>, Path(device_id): Path<i32>) -> Result<Json<R<String>>> {
    sqlx::query("DELETE FROM tb_device WHERE id = ?")
        .bind(device_id)
        .execute(&pool)
        .await?;

    Ok(Json(R::success()))
}

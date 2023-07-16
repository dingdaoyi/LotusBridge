use std::collections::HashMap;
use axum::extract::{Path, State};
use axum::Json;
use sqlx::SqlitePool;
use protocol_core::{Device, Point};
use crate::config::error::{EdgeError, Result};
use crate::models::device::{CreatDevice, CreateDeviceGroup, DeviceDTO, DeviceGroup};
use crate::models::R;

pub async fn load_all_device_details(pool: SqlitePool) -> Result<HashMap<String, Vec<Device>>> {
    let device_list = sqlx::query_as::<_, DeviceDTO>("SELECT * FROM tb_device")
        .fetch_all(&pool)
        .await?;
    let mut res: HashMap<String, Vec<Device>> = HashMap::new();
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
            protocol_name: device.protocol_name.clone(),
        };
        // 插入方式简洁处理
        res.entry(device.protocol_name.clone())
            .or_insert_with(Vec::new)
            .push(device_with_points);
    }
    tracing::info!("加载协议总数:{}",res.len());
    Ok(res)
}

pub async fn create_device(State(pool): State<SqlitePool>, device: Json<CreatDevice>) -> Result<Json<R<DeviceDTO>>> {
    let created_device = sqlx::query_as::<_, DeviceDTO>(
        "INSERT INTO tb_device (name, device_type, custom_data, protocol_name) VALUES (?, ?, ?, ?) RETURNING *",
    )
        .bind(&device.name)
        .bind(&device.device_type)
        .bind(sqlx::types::Json(&device.custom_data))
        .bind(device.protocol_name.clone())
        .fetch_one(&pool)
        .await?;

    Ok(Json(R::success_with_data(created_device)))
}

pub async fn get_device(State(pool): State<SqlitePool>, Path(id): Path<i32>) -> Result<Json<R<DeviceDTO>>> {
    let device = sqlx::query_as::<_, DeviceDTO>("SELECT * FROM tb_device WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?;
    match device {
        Some(device) => Ok(Json(R::success_with_data(device))),
        None => {
            // 没有找到匹配的行，返回自定义错误或其他逻辑
            Err(EdgeError::Message("设备不存在".into()))
        }
    }
}


pub async fn list_device(State(pool): State<SqlitePool>) -> Result<Json<R<Vec<DeviceDTO>>>> {
    let device = sqlx::query_as::<_, DeviceDTO>("SELECT * FROM tb_device WHERE")
        .fetch_all(&pool)
        .await?;
    Ok(Json(R::success_with_data(device)))
}

pub async fn update_device(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
    Json(device): Json<DeviceDTO>,
) -> Result<Json<R<String>>> {
    let updated_device = sqlx::query(
        "UPDATE tb_device SET name = $1, device_type = $2, custom_data = $3, protocol_name = $4 WHERE id = $5",
    )
        .bind(&device.name)
        .bind(&device.device_type)
        .bind(sqlx::types::Json(&device.custom_data))
        .bind(device.protocol_name)
        .bind(id)
        .execute(&pool)
        .await?;

    // 检查是否成功更新了设备
    if updated_device.rows_affected() > 0 {
        // 返回更新后的设备
        Ok(Json(R::success()))
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

pub async fn create_device_group(State(pool): State<SqlitePool>, device_group: Json<CreateDeviceGroup>) -> Result<Json<R<DeviceGroup>>> {
    let created_device_group = sqlx::query_as::<_, DeviceGroup>(
        "INSERT INTO tb_device_group (name, interval, device_id) VALUES (?, ?, ?) RETURNING *",
    )
        .bind(&device_group.name)
        .bind(device_group.interval)
        .bind(device_group.device_id)
        .fetch_one(&pool)
        .await.map_err(|_|EdgeError::Message("设备id不存在".into()))?;

    Ok(Json(R::success_with_data(created_device_group)))
}

pub async fn get_device_group(State(pool): State<SqlitePool>, Path(id): Path<i32>) -> Result<Json<R<DeviceGroup>>> {
    let device_group = sqlx::query_as::<_, DeviceGroup>("SELECT * FROM tb_device_group WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?;

    match device_group {
        Some(device_group) => Ok(Json(R::success_with_data(device_group))),
        None => Err(EdgeError::Message("设备组不存在".into())),
    }
}


pub async fn list_device_group(State(pool): State<SqlitePool>, Path(device_id): Path<i32>) -> Result<Json<R<Vec<DeviceGroup>>>> {
    let device_group_list = sqlx::query_as::<_, DeviceGroup>("SELECT * FROM tb_device_group WHERE device_id = ?")
        .bind(device_id)
        .fetch_all(&pool)
        .await?;
    Ok(Json(R::success_with_data(device_group_list)))
}

///定时任务启动
pub async fn list_all_device_group(pool: SqlitePool) -> Result<Vec<DeviceGroup>> {
    let device_group_list = sqlx::query_as::<_, DeviceGroup>("SELECT * FROM tb_device_group")
        .fetch_all(&pool)
        .await?;
    Ok(device_group_list)
}

pub async fn update_device_group(State(pool): State<SqlitePool>, Path(id): Path<i32>, device_group: Json<DeviceGroup>) -> Result<Json<R<String>>> {
    let updated_device_group = sqlx::query(
        "UPDATE tb_device_group SET name = $1, interval = $2, device_id = $3 WHERE id = $4",
    )
        .bind(&device_group.name)
        .bind(device_group.interval)
        .bind(device_group.device_id)
        .bind(id)
        .execute(&pool)
        .await?;

    if updated_device_group.rows_affected() > 0 {
        Ok(Json(R::success()))
    } else {
        Err(EdgeError::Message("设备组不存在".into()))
    }
}

pub async fn delete_device_group(State(pool): State<SqlitePool>, Path(id): Path<i32>) -> Result<Json<R<String>>> {
    let deleted_device_group = sqlx::query(
        "DELETE FROM tb_device_group WHERE id = $1",
    )
        .bind(id)
        .execute(&pool)
        .await?;

    if deleted_device_group.rows_affected() > 0 {
        Ok(Json(R::success()))
    } else {
        Err(EdgeError::Message("设备组不存在".into()))
    }
}

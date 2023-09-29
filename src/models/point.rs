use crate::models::page::PaginationRequest;
use protocol_core::{AccessMode, DataType};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CreatePoint {
    // 设备设备组id
    #[serde(rename = "groupId")]
    pub group_id: i32,
    //地址
    pub address: String,
    #[serde(rename = "dataType")]
    pub data_type: DataType,
    #[serde(rename = "accessMode")]
    pub access_mode: AccessMode,
    pub multiplier: f64,
    pub precision: u32,
    pub description: String,
    #[serde(rename = "partNumber")]
    pub part_number: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PointPageQuery {
    // 分页参数
    pub page: PaginationRequest,
    // 名称
    pub name: Option<String>,

    #[serde(rename = "groupId")]
    pub group_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct PointValue {
    // 分页参数
    pub page: PaginationRequest,
    // 名称
    pub name: Option<String>,

    #[serde(rename = "groupId")]
    pub group_id: i32,
}

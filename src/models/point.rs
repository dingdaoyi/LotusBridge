use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use protocol_core::{AccessMode, DataType};

#[derive(Debug, Serialize, Deserialize,FromRow)]
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

use std::any::Any;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow,Type};

/// 解析值
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    /// Integer value.
    Integer(i32),
    /// Float value.
    Float(f64),
    /// String value.
    String(String),
    /// Boolean value.
    Boolean(bool),
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Device {
    pub id: i32,
    pub name: String,
    #[serde(rename = "deviceType")]
    pub device_type: DeviceType,
    pub points: Vec<Point>,
    #[serde(rename = "customTata")]
    pub custom_data: HashMap<String, String>,
    #[serde(rename = "protocolId")]
    pub protocol_id:i32,
}

#[derive(Debug, Serialize, Deserialize,Type)]
pub enum DeviceType {
    #[serde(rename = "Gateway")]
    Gateway,
    #[serde(rename = "Independent")]
    Independent,
}

#[derive(Debug, Serialize, Deserialize,FromRow)]
pub struct Point {
    pub id: i32,
    // 设备id
    pub device_id: i32,
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

#[derive(Debug, Serialize, Deserialize,Type)]
// #[serde(untagged)]
pub enum DataType {
    #[serde(rename = "Integer")]
    Integer,
    #[serde(rename = "Float")]
    Float,
    #[serde(rename = "String")]
    String,
    #[serde(rename = "Boolean")]
    Boolean,
}

#[derive(Debug, Serialize, Deserialize,Type)]
pub enum AccessMode {
    #[serde(rename = "ReadWrite")]
    ReadWrite,
    #[serde(rename = "ReadOnly")]
    ReadOnly,
    #[serde(rename = "WriteOnly")]
    WriteOnly,
}

/// Protocol trait for data processing.
pub trait Protocol: Any + Send + Sync {
    ///读取点位数据
    fn read_point(&self, point_id: i64) -> Result<Value, String>;

    ///写点位,返回老点的值
    fn write_point(&self, point_id: i64, value: Value) -> Result<Value, String>;

    /// 初始化数据
    /// 后续添加参数 1, 点位,2 协议特有配置
    fn initialize(&self, device_list: Vec<Device>) -> Result<(), String>;

    /// 停止
    fn stop(&self, force: bool) -> Result<(), String>;

    /// 添加设备
    fn add_device(&self, device: Device) -> Result<(), String>;

    /// 删除设备
    fn remove_device(&self, device_id: i64) -> Result<(), String>;

    /// 更新设备
    fn update_device(&self, device: Device) -> Result<(), String>;
}

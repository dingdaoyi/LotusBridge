pub mod event_bus;
pub mod protocol_store;
use std::any::Any;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use std::error::Error;
use std::fmt;
use std::sync::{mpsc};
use derive_getters::Getters;
use crate::event_bus::PointEvent;

#[derive(Debug)]
struct ProtocolError(String);


impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ProtocolError {}


/// 解析值
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Value {
    /// Integer value.
    Integer(i32),
    /// Float value.
    Float(f64),
    /// Boolean value.
    Boolean(bool),
    //TODO 字符串无法实现copy,这儿先这么写,看有其他解决方案没
    String(String),
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Device {
    pub id: i32,
    pub name: String,
    #[serde(rename = "deviceType")]
    pub device_type: DeviceType,
    pub points: Vec<Point>,
    #[serde(rename = "customTata")]
    pub custom_data: HashMap<String, String>,
    #[serde(rename = "protocolName")]
    pub protocol_name: String,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum DeviceType {
    #[serde(rename = "Gateway")]
    Gateway,
    #[serde(rename = "Independent")]
    Independent,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Point {
    pub id: i32,
    // 设备id
    pub device_id: i32,

    // 设备id
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

#[derive(Debug, Serialize, Deserialize, FromRow,Clone)]
pub struct PointWithProtocolId {
    pub point_id: i32,
    pub device_id: i32,
    pub group_id: i32,
    pub address: String,
    pub data_type: DataType,
    pub access_mode: AccessMode,
    pub multiplier: f64,
    pub precision: u32,
    pub description: String,
    pub part_number: Option<String>,
    pub protocol_name: String,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum AccessMode {
    #[serde(rename = "ReadWrite")]
    ReadWrite,
    #[serde(rename = "ReadOnly")]
    ReadOnly,
    #[serde(rename = "WriteOnly")]
    WriteOnly,
}

#[derive(Getters)]
pub struct ReadPointRequest {
    pub device_id: i32,
    pub point_id: i32,
    //节点地址
    pub address: String,
    pub data_type: DataType,
    pub access_mode: AccessMode,
    pub multiplier: f64,
    pub precision: u32,

}

impl From<PointWithProtocolId>  for ReadPointRequest{
    fn from(value: PointWithProtocolId) -> Self {
        Self{
            device_id: value.device_id,
            point_id: value.point_id,
            address: value.address,
            data_type: value.data_type,
            access_mode: value.access_mode,
            multiplier: value.multiplier,
            precision: value.precision,
        }
    }
}
#[derive(Getters)]
pub struct WriterPointRequest {
    pub device_id: i32,
    pub point_id: i32,
    pub value: Value,
    pub address: String,
    pub data_type: DataType,
    pub access_mode: AccessMode,
    pub multiplier: f64,
    pub precision: u32,
}

impl From<PointWithProtocolId>  for WriterPointRequest{
    fn from(value: PointWithProtocolId) -> Self {
        Self{
            device_id: value.device_id,
            point_id: value.point_id,
            value: Value::Boolean(false),
            address: value.address,
            data_type: value.data_type,
            access_mode: value.access_mode,
            multiplier: value.multiplier,
            precision: value.precision,
        }
    }
}
/// Protocol trait for data processing.
pub trait Protocol: Any + Send + Sync {
    ///读取点位数据
    fn read_point(&self, request: ReadPointRequest) -> Result<Value, String>;

    ///写点位,返回老点的值
    fn write_point(&self, request: WriterPointRequest) -> Result<Value, String>;

    /// 初始化数据
    /// 后续添加参数 1, 点位,2 协议特有配置
    fn initialize(&mut self, device_list: Vec<Device>,
                  sender: mpsc::Sender<PointEvent>) -> Result<(), String>;

    /// 停止
    fn stop(&self, force: bool) -> Result<(), String>;    


    /// 添加设备
    fn add_device(&self, device: Device) -> Result<(), String>;

    /// 删除设备
    fn remove_device(&self, device_id: i64) -> Result<(), String>;

    /// 更新设备
    fn update_device(&self, device: Device) -> Result<(), String>;
}

pub mod event_bus;
pub mod protocol_store;
pub mod protocol_context;

use async_trait::async_trait;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use crate::protocol_context::ProtocolContext;

#[derive(Debug)]
pub struct ProtocolError(String);

impl ProtocolError {
    pub fn new<T: Into<String>>(msg: T) -> Self {
        Self(msg.into())
    }
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ProtocolError {}

impl From<&str> for ProtocolError {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for ProtocolError {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<std::io::Error> for ProtocolError {
    fn from(value: std::io::Error) -> Self {
        Self(value.to_string())
    }
}

/// 解析值
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Value {
    /// Integer value.
    Integer(i32),
    /// Long value.
    Long(i64),
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
    #[serde(rename = "deviceId")]
    pub device_id: i32,

    // 设备id
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
    //标识符
    pub identifier: Option<String>,

    pub description: String,
    #[serde(rename = "partNumber")]
    pub part_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct PointWithProtocolId {
    pub point_id: i32,
    pub device_id: i32,
    pub group_id: i32,
    pub address: String,
    pub data_type: DataType,
    pub access_mode: AccessMode,
    //标识符
    pub identifier: Option<String>,
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
    #[serde(rename = "Long")]
    Long,
    #[serde(rename = "Float")]
    Float,
    #[serde(rename = "String")]
    String,
    #[serde(rename = "Boolean")]
    Boolean,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, Eq, PartialEq)]
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

impl From<PointWithProtocolId> for ReadPointRequest {
    fn from(value: PointWithProtocolId) -> Self {
        Self {
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

impl From<PointWithProtocolId> for WriterPointRequest {
    fn from(value: PointWithProtocolId) -> Self {
        Self {
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

/// 协议状态
#[derive(Debug, Type, Clone, Copy)]
pub enum ProtocolState {
    /// 未初始化
    NoInitialized,
    /// 运行中
    Running,
    /// 停止
    Closed,
}

/// Protocol trait for data processing.
#[async_trait]
pub trait Protocol: Any + Send + Sync {

    fn context(&self) -> Option<ProtocolContext>;

    /// 初始化数据
    /// 后续添加参数 1, 点位,2 协议特有配置
    async fn initialize(
        &mut self,
        context: ProtocolContext,
    ) -> Result<(), ProtocolError>;

    ///读取点位数据
    async fn read_point(&self, request: ReadPointRequest) -> Result<Value, ProtocolError>;

    ///写点位,返回老点的值
    async fn write_point(&self, request: WriterPointRequest) -> Result<Value, ProtocolError>;

    fn get_state(&self) -> ProtocolState{
        match self.context() {
            None => {
                ProtocolState::NoInitialized
            }
            Some(ctx) => {
                ctx.status()
            }
        }
    }


    /// 停止
    fn stop(&mut self, force: bool) -> Result<(), ProtocolError>;

    /// 添加设备
    fn add_device(&self, device: Device) -> Result<(), ProtocolError> {
        match self.context() {
            None => {
                return Err(ProtocolError::new("context 为空"));
            }
            Some(ctx) => {
                ctx.add_device(device)?;
            }
        };
        Ok(())
    }

    /// 删除设备
    fn remove_device(&self, device_id: i32) -> Result<(), ProtocolError> {
        match self.context() {
            None => {
                return Err(ProtocolError::new("context 为空"));
            }
            Some(ctx) => {
                ctx.remove_device(device_id)?;
            }
        };
        Ok(())
    }

    /// 更新设备
    fn update_device(&self, device: Device) -> Result<(), ProtocolError> {
        match self.context() {
            None => {
                return Err(ProtocolError::new("context 为空"));
            }
            Some(ctx) => {
                ctx.update_device(device)?;
            }
        };
        Ok(())
    }
}

/// int  转换为  long
pub fn combine_u16_to_u32(high: u16, low: u16) -> u32 {
    let high_shifted = (high as u32) << 16;
    let low = low as u32;
    let combined = high_shifted | low;
    combined
}

/// long 转int
pub fn split_u32_to_u16s(data: u32) -> (u16, u16) {
    let high = (data >> 16) as u16;
    let low = data as u16;
    (high, low)
}
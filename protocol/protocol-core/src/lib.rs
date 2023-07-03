use std::any::Any;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Represents the result of a decoder.
#[derive(Debug, Serialize, Deserialize)]
pub struct DecoderResult {
    /// List of decoder data items.
    pub decoder_data_item_list: Vec<DecoderDataItem>,
    /// ACK Message.
    pub ack_message: Option<String>,
    /// Message ID.
    pub message_id: Option<i32>,
    /// Driver service name.
    pub driver_service_name: Option<String>,
    /// Driver name.
    pub driver_name: Option<String>,
    /// IMSI.
    pub imsi: Option<String>,
    /// ACK topic.
    pub ack_topic: Option<String>,
    /// ICCID.
    pub iccid: Option<String>,
    /// Third-party device ID.
    pub trd_device_id: Option<String>,
    /// Source address.
    pub source_address: Option<String>,
}

/// Represents a decoder data item.
#[derive(Debug, Serialize, Deserialize)]
pub struct DecoderDataItem {
    /// Identifier.
    pub identifier: String,
    /// Unit address.
    pub unit_address: String,
    /// Unit type.
    pub unit_type: Option<i32>,
    /// Unit type name.
    pub unit_type_name: Option<String>,
    /// System type.
    pub system_type: Option<i32>,
    /// Unit description.
    pub unit_description: Option<String>,
    /// Value.
    pub value: Option<Value>,
    /// System address.
    pub system_address: Option<i32>,
    /// Child device code.
    pub child_device_code: Option<String>,
    /// Acquisition time.
    pub acquisition_time: Option<NaiveDateTime>,
}

/// Represents a value that can be stored in `DecoderDataItem`.
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

pub struct Device {
    id: i64,
    name: String,
    device_type: DeviceType,
    points: Vec<Point>,
}

enum DeviceType {
    Gateway,
    Independent,
}

struct Point {
    id: i64,
    address: String,
    data_type: DataType,
    access_mode: AccessMode,
    multiplier: f64,
    precision: u32,
    description: String,
    part_number: Option<String>,
}

enum DataType {
    // 定义不同的数据类型
}

enum AccessMode {
    ReadWrite,
    ReadOnly,
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

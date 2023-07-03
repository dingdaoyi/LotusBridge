use std::collections::HashMap;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use sqlx::types::Json;
use protocol_core::{Device, DeviceType, Point, Value};

#[derive(Debug, Deserialize)]
pub struct CreatDevice {
    pub name: String,
    #[serde(rename = "deviceType")]
    pub device_type: DeviceType,
    #[serde(rename = "customTata")]
    pub custom_data: Json<HashMap<String, String>>,
    #[serde(rename = "protocolId")]
    pub protocol_id: i32,
}

#[derive(Debug, Serialize,Deserialize, FromRow)]
pub struct DeviceDTO {
    pub id: i32,
    pub name: String,
    #[serde(rename = "deviceType")]
    pub device_type: DeviceType,
    #[serde(rename = "customTata")]
    pub custom_data: Json<HashMap<String, String>>,
    #[serde(rename = "protocolId")]
    pub protocol_id: i32,
}

impl From<DeviceDTO> for Device {
    fn from(value: DeviceDTO) -> Self {
        Self {
            id: value.id,
            name: value.name,
            device_type: value.device_type,
            points: vec![],
            custom_data: value.custom_data.0,
            protocol_id: value.protocol_id,
        }
    }
}
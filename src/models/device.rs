use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use sqlx::types::Json;
use export_core::model::PointValue;
use protocol_core::{Device, DeviceType};

#[derive(Debug, Deserialize)]
pub struct CreatDevice {
    pub name: String,
    #[serde(rename = "deviceType")]
    pub device_type: DeviceType,
    #[serde(rename = "customTata")]
    pub custom_data: Json<HashMap<String, String>>,
    #[serde(rename = "protocolName")]
    pub protocol_name: String,
}


#[derive(Debug, Serialize,Deserialize, FromRow)]
pub struct DeviceDTO {
    pub id: i32,
    pub name: String,
    #[serde(rename = "deviceType")]
    pub device_type: DeviceType,
    #[serde(rename = "customTata")]
    pub custom_data: Json<HashMap<String, String>>,
    #[serde(rename = "protocolName")]
    pub protocol_name: String,
}

impl From<DeviceDTO> for Device {
    fn from(value: DeviceDTO) -> Self {
        Self {
            id: value.id,
            name: value.name,
            device_type: value.device_type,
            points: vec![],
            custom_data: value.custom_data.0,
            protocol_name: value.protocol_name,
        }
    }
}

#[derive(Debug, Deserialize,Serialize,FromRow, Clone)]
pub struct DeviceGroup {
    pub id: i32,
    pub name: String,
    pub interval: i32,
    pub device_id: i32,
}


#[derive(Debug, Deserialize)]
pub struct CreateDeviceGroup {
    pub name: String,
    pub interval: i32,
    pub device_id: i32,
}


#[derive(Debug, Clone)]
pub struct DeviceGroupValue {
    pub id: i32,
    pub name: String,
    pub device_id: i32,
    pub point_values: Vec<PointValue>
}

impl From<DeviceGroup> for DeviceGroupValue{
    fn from(device_group: DeviceGroup) -> Self {
        Self{
            id: device_group.id,
            name: device_group.name,
            device_id: device_group.device_id,
            point_values: vec![],
        }
    }
}




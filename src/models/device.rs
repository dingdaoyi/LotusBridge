use export_core::model::DeviceGroupValue;
use protocol_core::{Device, DeviceType};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::FromRow;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct CreatDevice {
    pub name: String,
    #[serde(rename = "deviceType")]
    pub device_type: DeviceType,
    #[serde(rename = "customData")]
    pub custom_data: Json<HashMap<String, String>>,
    #[serde(rename = "protocolName")]
    pub protocol_name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DeviceDTO {
    pub id: i32,
    pub name: String,
    #[serde(rename = "deviceType")]
    pub device_type: DeviceType,
    #[serde(rename = "customData")]
    pub custom_data: Json<HashMap<String, String>>,
    #[serde(rename = "protocolName")]
    pub protocol_name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DeviceDTOStatistics {
    pub id: i32,
    pub name: String,
    #[serde(rename = "deviceType")]
    pub device_type: DeviceType,
    #[serde(rename = "customData")]
    pub custom_data: Json<HashMap<String, String>>,
    #[serde(rename = "protocolName")]
    pub protocol_name: String,
    #[serde(rename = "groupCount")]
    pub group_count: u16,

    #[serde(rename = "pointCount")]
    pub point_count: u16,
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

#[derive(Debug, Deserialize, Serialize, FromRow, Clone)]
pub struct DeviceGroup {
    pub id: i32,
    pub name: String,
    pub interval: i32,
    pub device_id: i32,
}

#[derive(Debug, Deserialize, Serialize, FromRow, Clone)]
pub struct DeviceGroupWithExportName {
    pub id: i32,
    pub name: String,
    pub interval: i32,
    pub device_id: i32,
    pub export_name: Vec<String>,
}

impl From<DeviceGroup> for DeviceGroupWithExportName {
    fn from(device_group: DeviceGroup) -> Self {
        Self {
            id: device_group.id,
            name: device_group.name,
            interval: device_group.interval,
            device_id: device_group.device_id,
            export_name: vec![],
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateDeviceGroup {
    pub name: String,
    pub interval: i32,
    pub device_id: i32,
}

impl From<DeviceGroup> for DeviceGroupValue {
    fn from(device_group: DeviceGroup) -> Self {
        Self {
            id: device_group.id,
            name: device_group.name,
            device_id: device_group.device_id,
            point_values: vec![],
            export_name: vec![],
        }
    }
}

impl From<DeviceGroupWithExportName> for DeviceGroupValue {
    fn from(device_group: DeviceGroupWithExportName) -> Self {
        Self {
            id: device_group.id,
            name: device_group.name,
            device_id: device_group.device_id,
            point_values: vec![],
            export_name: device_group.export_name,
        }
    }
}

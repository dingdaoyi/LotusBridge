use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;
use protocol_core::{Point, PointWithProtocolId, Value};

#[derive(Debug, Clone)]
pub struct DeviceGroupValue {
    pub id: i32,
    pub name: String,
    pub device_id: i32,
    pub point_values: Vec<PointValue>
}


#[derive(Debug, Clone)]
pub struct PointValue {
    pub id: i32,
    pub point:Point,
    pub value: Option<Value>
}

impl From<Point> for PointValue{
    fn from(point: Point) -> Self {
        Self{
            id:point.id,
            point,
            value:None
        }
    }
}
impl From<PointWithProtocolId> for PointValue{
    fn from(point: PointWithProtocolId) -> Self {
        Self{
            id:point.point_id,
            point:Point{
                id: point.point_id,
                device_id: point.device_id,
                group_id: point.group_id,
                address: point.address,
                data_type: point.data_type,
                access_mode: point.access_mode,
                multiplier: point.multiplier,
                precision: point.precision,
                description: point.description,
                part_number: point.part_number,
            },
            value:None
        }
    }
}

/// 数据导出配置
#[derive(Debug,Serialize,Deserialize,FromRow,Clone)]
pub struct ExportConfig {
    pub id: i32,
    //推送名称
    pub name: String,
    // 推送配置
    pub configuration: Json<HashMap<String, String>>,
    // 描述
    pub description : String,

    #[serde(rename = "pluginId")]
    pub plugin_id: i32,
}

/// 数据导出配置
#[derive(Debug,Deserialize,Clone)]
pub struct CreateExportConfig {
    pub name: String,
    // 推送配置
    pub configuration: Json<HashMap<String, String>>,
    // 描述
    pub description : String,

    #[serde(rename = "pluginId")]
    pub plugin_id: i32,
}
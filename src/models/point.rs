use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use protocol_core::{AccessMode, DataType, Point, PointWithProtocolId, Value};

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
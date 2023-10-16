use crate::Value;
use serde::{Deserialize, Serialize};

/// 点位事件
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PointEvent {
    /// 点位号
    pub point_id: i32,
    pub value: Value,
}
impl PointEvent {
    pub fn new(point_id: i32, value: Value) -> Self {
        Self { point_id, value }
    }
}

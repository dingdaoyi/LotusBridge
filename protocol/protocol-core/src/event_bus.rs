use serde::{Deserialize, Serialize};
use crate::Value;

/// 点位事件
#[derive(Clone, Debug,Serialize,Deserialize)]
pub struct PointEvent {
    /// 点位号
    pub point_id: i32,
    pub value: Value,
}

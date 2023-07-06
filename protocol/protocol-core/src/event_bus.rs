use crate::Value;

/// 点位事件
#[derive(Clone, Debug, Copy)]
pub struct PointEvent {
    /// 点位号
    pub point_id: i32,
    pub value: Value,
}

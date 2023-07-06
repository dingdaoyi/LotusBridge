use std::sync::{Arc, Mutex, RwLock};
use pharos::*;
use once_cell::sync::OnceCell;
use crate::{ProtocolError, Value};

static PUBSUB_STORE: OnceCell<Arc<RwLock<PharosPubSubModel>>> = OnceCell::new();

/// 获取或创建 PharosPubSubModel 实例
pub fn get_pubsub_model() -> Option<Arc<RwLock<PharosPubSubModel>>> {
    PUBSUB_STORE.get().cloned()
}

pub fn set_pharos_pub_sub_model(pharos_pub_sub_model: PharosPubSubModel) -> Result<(), String> {
    let res = PUBSUB_STORE.set(Arc::new(RwLock::new(pharos_pub_sub_model)));
    match res {
        Ok(_) => Ok(()),
        Err(_) => Err("重复设置pharos_pub_sub_model".to_string())
    }
}

/// 点位事件
#[derive(Clone, Debug, Copy)]
pub struct PointEvent {
    /// 点位号
    pub point_id: i32,
    pub value: Value,
}

/// 基于Pharos实现的发布订阅模型
#[derive(Clone)]
pub struct PharosPubSubModel {
    pub pharos: SharedPharos<PointEvent>,
}

impl PharosPubSubModel {
    pub fn new() -> PharosPubSubModel {
        Self {
            pharos: SharedPharos::default(),
        }
    }

    pub async fn publish(&mut self, event: PointEvent) -> Result<(), String> {
        let res = self.pharos.notify(event).await;
        res.map_err(|_| "发送消息失败".into())
    }
}

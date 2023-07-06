use protocol_core::event_bus::PointEvent;
use crate::config::cache::get_protocol_store;
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use lazy_static::lazy_static;
use protocol_core::{Device, Value};

// 全局缓存数据结构
lazy_static! {
    static ref EVENT_CACHE: Arc<Mutex<HashMap<i32, PointEvent>>> = Arc::new(Mutex::new(HashMap::new()));
}


// 处理上报逻辑
pub async fn handler_event(event: PointEvent) {
    tracing::debug!("点位:{},值:{:?}", event.point_id, event.value);
    // 更新缓存
    let mut cache = EVENT_CACHE.lock().unwrap();
    cache.insert(event.point_id, event.clone());
}

fn read_from_cache(point_id: i32) -> Option<PointEvent> {
    // 从缓存中读取数据
    let cache = EVENT_CACHE.lock().unwrap();
    cache.get(&point_id).cloned()
}

fn read_from_protocol(point_id: i32,protocol_id :i32) -> Option<PointEvent> {
    let store = get_protocol_store().unwrap();
    let protocol_mutex = store.get_protocol(protocol_id).unwrap();
    // //为啥连在一起会报错? 所有权问题 需要细细品尝.为撒别的地方没有释放锁
    let binding = protocol_mutex?;
    // // 这儿会卡住.
    let protocol = binding.lock().unwrap();
    // let protocol = protocol_mutex?.lock().unwrap();

    match protocol.read_point(point_id) {
        Ok(value) => Some(PointEvent {
            point_id,
            value,
        }),
        Err(msg) => {
            tracing::error!("读取point数据错误:{}",msg);
            None
        }
    }
}

pub fn read_point(point_id: i32,protocol_id :i32) -> Option<PointEvent> {
    // 先尝试从缓存中读取数据
    let cached_event = read_from_cache(point_id);
    if cached_event.is_some() {
        return cached_event;
    }
    // 缓存中没有数据，则调用协议API读取数据
    read_from_protocol(point_id,protocol_id)
}

use protocol_core::event_bus::PointEvent;
use crate::config::cache::get_protocol_store;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use protocol_core::ReadPointRequest;
use crate::config::error::EdgeError;

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

async fn read_from_protocol(protocol_name :String, request:ReadPointRequest) -> Result<PointEvent,EdgeError> {
    let store = get_protocol_store().unwrap();
    let protocol_mutex=store.get_protocol(protocol_name.clone());
    
    match  protocol_mutex{
        Some(protocol_mutex) => {
            let protocol = protocol_mutex.read().unwrap();
            let point_id=request.point_id.clone();
            match protocol.read_point(request).await {
                Ok(value) => Ok(PointEvent {
                    point_id,
                    value,
                }),
                Err(msg) => {
                    tracing::error!("读取point数据错误:{}",msg);
                    Err(EdgeError::Message(msg))
                }
            }
        }
        None=> {
            Err(EdgeError::Message(format!("协议:{}不存在",&protocol_name)))
        }
    }
}

pub async fn read_point(protocol_name :String, request:ReadPointRequest) -> Result<PointEvent,EdgeError> {
    // 先尝试从缓存中读取数据
    let cached_event = read_from_cache(request.point_id);
    if cached_event.is_some() {
        return Ok(cached_event.unwrap());
    }
    // 缓存中没有数据，则调用协议API读取数据
    read_from_protocol(protocol_name,request).await
}

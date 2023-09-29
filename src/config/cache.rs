use crate::config::error::{EdgeError, Result};
use export_core::export_store::DataExportStore;
use protocol_core::event_bus::PointEvent;
use protocol_core::protocol_store::ProtocolStore;
use protocol_core::{Device, Protocol, ProtocolError};
use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;

static PROTOCOL_STORE: OnceLock<ProtocolStore> = OnceLock::new();
static EXPORT_STORE: OnceLock<DataExportStore> = OnceLock::new();

pub fn set_protocol_store(protocol_store: ProtocolStore) -> Result<()> {
    let res = PROTOCOL_STORE.set(protocol_store);
    match res {
        Ok(_) => Ok(()),
        Err(_) => Err(EdgeError::Message("重复设置ProtocolStore".to_string())),
    }
}

pub fn get_export_store() -> Option<&'static DataExportStore> {
    let res = EXPORT_STORE.get_or_init(|| DataExportStore::new());
    Some(res)
}

pub fn get_protocol_store() -> Option<&'static ProtocolStore> {
    PROTOCOL_STORE.get()
}

pub async fn initialize_protocol(
    name: String,
    sender: tokio::sync::mpsc::Sender<PointEvent>,
    device_list: Vec<Device>,
) -> Result<()> {
    tracing::debug!("开始初始化协议设备:{:?}", name);
    let store = get_protocol_store().unwrap();
    let mut map = store.clone().inner;
    let protocol = map.lock().await.get_mut(name.as_str()).cloned();
    match protocol {
        None => Err(EdgeError::Message(format!("协议:{}不存在", &name))),
        Some(protocol) => {
            protocol
                .lock()
                .await
                .initialize(device_list, sender)
                .await?;
            tracing::debug!("结束初始化协议:{:?}", name);
            Ok(())
        }
    }
}

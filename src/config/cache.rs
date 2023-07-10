use std::sync::mpsc;
use once_cell::sync::OnceCell;
use protocol_core::event_bus::PointEvent;
use protocol_core::Device;
use protocol_core::protocol_store::ProtocolStore;
use crate::config::error::{EdgeError, Result};

static PROTOCOL_STORE: OnceCell<ProtocolStore> = OnceCell::new();

pub fn set_protocol_store(protocol_store: ProtocolStore) -> Result<()> {
    let res = PROTOCOL_STORE.set(protocol_store);
    match res {
        Ok(_) => Ok(()),
        Err(_) => Err(EdgeError::Message("重复设置ProtocolStore".to_string()))
    }
}

pub fn get_protocol_store() -> Option<&'static ProtocolStore> {
    PROTOCOL_STORE.get()
}

pub async fn initialize_protocol(
    name: String,
    sender: mpsc::Sender<PointEvent>,
    device_list: Vec<Device>,
) -> Result<()> {
    let store = get_protocol_store().unwrap();
    let map = store.clone().inner;
    let mut res = map.write().unwrap();
    let protocol = res.get_mut(name.as_str()).unwrap();
    protocol.write().unwrap().initialize(device_list, sender)?;
    Ok(())
}

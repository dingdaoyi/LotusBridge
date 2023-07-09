use std::collections::HashMap;
use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, mpsc, Mutex, RwLock, RwLockReadGuard};

use libloading::{Library, Symbol};
use once_cell::sync::OnceCell;

use protocol_core::event_bus::PointEvent;
use protocol_core::{Device, Protocol};
use protocol_core::protocol_store::ProtocolStore;

use crate::config::error::{EdgeError, Result};
use crate::models::plugin::{get_library_filename, ProtocolConfig};

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
    protocol.write().unwrap().initialize(device_list, sender).await?;
    Ok(())
}

pub async fn start_protocol() -> Result<()> {
    let store = get_protocol_store().unwrap();
    let map = store.clone().inner;
    let res = map.read().unwrap();
    for (protocol_name, protocol) in res.iter() {
        let protocol_arc = Arc::clone(protocol);
        tokio::spawn(async move {
            let protocol_lock = protocol_arc.lock().unwrap();
            let protocol_box = &*protocol_lock;
            // 在需要引用的范围内执行操作
            protocol_box.start().await.unwrap();
            // 作用域结束，自动释放锁
        });
    }
    Ok(())
}




enum ProtocolCo {
    #[cfg(feature = "modbus-tcp")]
    ModbusTcp(protocol_modbus_tcp::ModbusTcpProtocol),
    #[cfg(feature = "protocol1")]
    Protocol1(Protocol1),
    // 添加其他协议的成员...
}


use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};
use libloading::{Library, Symbol};
use tokio::sync::RwLockWriteGuard;
use protocol_core::Protocol;
use crate::config::error::{EdgeError, Result};
use crate::models::plugin::ProtocolConfig;

#[derive(Clone)]
pub struct ProtocolStore {
    /// key为协议名称, value为协议
    inner: Arc<RwLock<HashMap<String, Arc<Box<dyn Protocol>>>>>,
}

impl ProtocolStore {
    /// 创建
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 添加协议
    pub fn add_protocol(&self, name: String, protocol: Box<dyn Protocol>) -> Result<()> {
        let protocol_mutex  = Arc::new(protocol);
        self.inner.write()?.insert(name, protocol_mutex);
        Ok(())
    }

    pub fn load_protocol(&self, config: &ProtocolConfig) -> Result<()> {
        // 加载协议库
        let lib_path = Path::new(&config.path);
        let lib = unsafe {
            Library::new(lib_path)
        }?;
        // 获取 create_protocol 函数符号
        type CreateProtocolFn = extern "C" fn() -> *mut dyn Protocol;
        let constructor: Symbol<CreateProtocolFn> = unsafe {
            lib.get(b"create_protocol")?
        };
        // 调用该函数，取得 Protocol Trait 实例的原始指针
        let boxed_raw = constructor();
        // 通过原始指针构造 Box
        let protocol_box = unsafe {
            Box::from_raw(boxed_raw)
        };
        self.add_protocol(config.name.clone(), protocol_box)
    }

    /// 清空协议
    pub fn clear_protocols(&self) -> Result<()> {
        self.inner.write()?.clear();
        Ok(())
    }

    /// 删除指定名称的协议
    pub fn remove_protocol(&self, name: &str) -> Result<()> {
        self.inner.write()?.remove(name);
        Ok(())
    }

    /// 根据名称获取协议
    pub fn get_protocol(&self, name: &str) -> Result<Option<Arc<Box<dyn Protocol>>>> {
       Ok(self.inner.read()?.get(name).cloned())
    }
}

use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, RwLock};
use libloading::{Library, Symbol};
use once_cell::sync::OnceCell;
use protocol_core::Protocol;
use crate::config::error::{EdgeError, Result};
use crate::models::plugin::ProtocolConfig;

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

#[derive(Clone)]
pub struct ProtocolStore {
    // key为协议名称, value为协议
    inner: Arc<RwLock<HashMap<String, Arc<Box<dyn Protocol>>>>>,
    //父级路径
    lib_path: String,
}

impl ProtocolStore {
    /// 创建
    pub fn new(lib_path: String) -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
            lib_path,
        }
    }

    /// 添加协议
    pub fn add_protocol(&self, name: String, protocol: Box<dyn Protocol>) -> Result<()> {
        tracing::info!("加载协议:{}",&name);
        let protocol_mutex = Arc::new(protocol);
        self.inner.write()?.insert(name, protocol_mutex);
        Ok(())
    }

    pub fn load_protocol(&self, config: &ProtocolConfig) -> Result<()> {
        // 加载协议库
        let lib_path = Path::new(&self.lib_path);
        let protocol_path = lib_path.join(&config.path);
        let lib = unsafe {
            Library::new(protocol_path)
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

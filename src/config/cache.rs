use std::collections::HashMap;
use std::ops::DerefMut;
use std::path::Path;
use std::sync::{Arc, mpsc, Mutex};

use libloading::{Library, Symbol};
use once_cell::sync::OnceCell;

use protocol_core::event_bus::PointEvent;
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
    // key为协议ID, value为协议
    inner: Arc<Mutex<HashMap<i32, Arc<Mutex<Box<dyn Protocol>>>>>>,
    //父级路径
    lib_path: String,
}

impl ProtocolStore {
    /// 创建
    pub fn new(lib_path: String) -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
            lib_path,
        }
    }

    /// 添加协议
    pub fn add_protocol(&self, id: i32, protocol: Box<dyn Protocol>) -> Result<()> {
        tracing::info!("加载协议: {}", id);
        let protocol_mutex = Arc::new(Mutex::new(protocol));
        self.inner.lock()?.insert(id, protocol_mutex);
        Ok(())
    }

    pub async fn load_protocol(&self, config: &ProtocolConfig) -> Result<()> {
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
        self.add_protocol(config.id, protocol_box)
    }

    /// 清空协议
    pub fn clear_protocols(&self) -> Result<()> {
        self.inner.lock()?.clear();
        Ok(())
    }

    /// 删除指定ID的协议
    pub fn remove_protocol(&self, id: i32) -> Result<()> {
        self.inner.lock()?.remove(&id);
        Ok(())
    }

    /// 根据ID获取协议
    pub fn get_protocol(&self, id: i32) -> Result<Option<Arc<Mutex<Box<dyn Protocol>>>>> {
        Ok(self.inner.lock()?.get(&id).cloned())
    }


    /// 当使用 Arc<Mutex<_>> 时，Arc 提供了共享所有权的功能，而 Mutex 提供了线程间同步访问的能力。Arc 记录了共享的引用计数，以便多个线程可以持有对数据的共享引用。
    /// Mutex 则允许在任意给定时间内，只有一个线程可以拥有对数据的独占访问权。当一个线程获取了 Mutex 的锁时，其他线程将被阻塞，直到该线程释放锁。
    /// 但是，在异步任务中，如果直接将 Arc<Mutex<_>> 的引用传递给异步任务，任务会一直持有锁，无法释放。这是因为异步任务是非阻塞的，不会主动释放锁。
    /// 为了解决这个问题，我们使用 Arc::try_unwrap 尝试将 Arc<Mutex<_>> 转换为 Mutex<_>。这个转换只有在当前是最后一个持有 Arc<Mutex<_>> 的引用时才会成功。成功后，我们就可以在异步任务中操作 Mutex，因为异步任务结束后，它会释放 Mutex 的所有权，从而释放锁。
    /// 需要注意的是，Arc::try_unwrap 的成功与否取决于引用计数的状态。如果在转换时引用计数不为 1（即有其他线程持有引用），转换将失败并返回 Err。因此，在使用 Arc::try_unwrap 之前，需要确保只有一个线程持有 Arc<Mutex<_>> 的引用。
    pub fn init_protocol(&self, sender: mpsc::Sender<PointEvent>) -> Result<()> {
        for protocol in self.inner.lock()?.values() {
            let protocol_mutex = protocol.clone();
            let sender = sender.clone();
            tokio::task::spawn(async move {
                // 使用 Arc::try_unwrap 尝试将 Arc<Mutex<_>> 转换为 Mutex<_>，如果成功转换，就说明当前任务是最后一个持有 Arc<Mutex<_>> 的任务。
                if let Ok(protocol_mutex) = Arc::try_unwrap(protocol_mutex) {
                    let mut protocol_mutex = protocol_mutex.lock().unwrap();
                    protocol_mutex.deref_mut().initialize(vec![], sender).unwrap();
                }
            });
        }
        Ok(())
    }
}

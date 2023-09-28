use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::Protocol;

#[derive(Clone)]
pub struct ProtocolStore {
    pub inner: Arc<RwLock<HashMap<String, Arc<RwLock<Box<dyn Protocol>>>>>>,
}

impl ProtocolStore {
    pub fn register_protocol(&self, protocol_name: String, protocol: impl Protocol) {
        let protocol_box: Box<dyn Protocol> = Box::new(protocol);
        let protocol_arc = Arc::new(RwLock::new(protocol_box));
        let mut store = self.inner.write().unwrap();
        store.insert(protocol_name, protocol_arc);
    }

    pub fn get_protocol(&self, protocol_name: String) -> Option<Arc<RwLock<Box<dyn Protocol>>>> {
        let result = self.inner.read();
        match result {
            Ok(map) => {
                map.get(&protocol_name).cloned()
            }
            Err(msg) => {
                println!("获取协议失败:{}:{}",&protocol_name,msg);
                None
            }
        }

    }
    pub fn  new()->Self {
        Self{
            inner: Arc::new(Default::default()),
        }
    }
}

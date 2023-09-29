use crate::Protocol;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ProtocolStore {
    pub inner: Arc<Mutex<HashMap<String, Arc<Mutex<Box<dyn Protocol>>>>>>,
}

impl ProtocolStore {
    pub async fn register_protocol(&self, protocol_name: String, protocol: impl Protocol) {
        let protocol_box: Box<dyn Protocol> = Box::new(protocol);
        let protocol_arc = Arc::new(Mutex::new(protocol_box));
        let mut store = self.inner.lock().await;
        store.insert(protocol_name, protocol_arc);
    }

    pub async fn get_protocol(
        &self,
        protocol_name: String,
    ) -> Option<Arc<Mutex<Box<dyn Protocol>>>> {
        let result = self.inner.lock().await;
        let res = result.get(&protocol_name).cloned();
        res
    }
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Default::default()),
        }
    }
}

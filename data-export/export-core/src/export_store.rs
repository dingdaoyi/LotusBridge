use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::DataExport;

#[derive(Clone)]
pub struct DataExportStore {
    pub inner: Arc<RwLock<HashMap<String, Arc<RwLock<Box<dyn DataExport>>>>>>,
}

impl DataExportStore {
    pub fn register_data_export(&self, data_export_name: String, data_export: impl DataExport) {
        let data_export_box: Box<dyn DataExport> = Box::new(data_export);
        let data_export_arc = Arc::new(RwLock::new(data_export_box));
        let mut store = self.inner.write().unwrap();
        store.insert(data_export_name, data_export_arc);
    }

    pub fn get_data_export(&self, protocol_name: String) -> Option<Arc<RwLock<Box<dyn DataExport>>>> {
        let map = self.inner.read().unwrap();
        map.get(&protocol_name).cloned()
    }
    pub fn  new()->Self {
        Self{
            inner: Arc::new(Default::default()),
        }
    }
}

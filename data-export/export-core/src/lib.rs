pub mod model;
pub mod export_store;

use std::any::Any;
use crate::model::{DeviceGroupValue, ExportConfig};

///北向输出配置
pub trait DataExport: Any + Send + Sync {

    /// 初始化
    fn initialize(&mut self, config: ExportConfig) -> Result<(), String>;

    /// 停止
    fn stop(&self, force: bool) -> Result<(), String>;

    ///导出数据
    fn export(&self, device_group_value:DeviceGroupValue) ->Result<(), String>;
}

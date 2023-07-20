use crate::config::cache::get_export_store;
use crate::config::error::Result;
//初始化协议
pub(crate) async fn init_data_export() -> Result<()> {
    register_all_export_data().await;
    Ok(())
}

pub async fn register_all_export_data() {
    let store = get_export_store().unwrap();

    #[cfg(feature = "xiaozhiyun-push")]
    {
      let data_export: export_xiaozhiyun::XiaozhiyunDataExport= Default::default();
        store.register_data_export("xiaozhiyun-push".to_string(), data_export);
    }
}
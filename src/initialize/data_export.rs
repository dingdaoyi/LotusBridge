use std::collections::HashMap;
use export_core::model::{ExportConfig, ExportConfigWithPluginName};
use crate::config::cache::get_export_store;
use crate::config::error::Result;
use crate::handler::export_config_handler::load_all_export_config;

//初始化协议
pub(crate) async fn init_data_export() -> Result<()> {
    register_all_export_data().await;

    let export_config_list: Vec<ExportConfigWithPluginName> = load_all_export_config().await?;
    let export_config_map: HashMap<String, Vec<ExportConfig>> = export_config_list.iter()
        .fold(HashMap::new(),|mut map, export_config|{
            map.entry(export_config.plugin_name.clone())
                .and_modify(|list|list.push(export_config.clone().into()))
                .or_insert_with(||vec![export_config.clone().into()]);
            map
        });
    let store = get_export_store().unwrap();
    let map = store.inner.write().await;
    for (plugin_name,export_config_list) in export_config_map.iter() {
        // initialize_protocol(protocol_name.clone(), sender.clone(), device_list.to_vec()).await?;
        let value = map.get(plugin_name);
       let mut export_config = value.unwrap().write();
       let res= export_config.await.initialize(export_config_list.clone());
        match res {
            Ok(_) => {}
            Err(msg) => {
                tracing::error!("初始化export失败:{}|{}",plugin_name,msg);
            }
        }
        //初始化数据
    }
    Ok(())
}

pub async fn register_all_export_data() {
    let store = get_export_store().unwrap();

    #[cfg(feature = "xiaozhiyun-push")]
    {
      let data_export: export_xiaozhiyun::XiaozhiyunDataExport= Default::default();
        store.register_data_export("xiaozhiyun-push".to_string(), data_export).await;
    }
}
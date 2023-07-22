use std::collections::HashMap;
use export_core::model::ExportConfig;
use crate::config::cache::get_export_store;
use crate::config::error::Result;
use crate::handler::export_config_handler::load_all_export_config;

//初始化协议
pub(crate) async fn init_data_export() -> Result<()> {
    register_all_export_data().await;

    let export_config_list: Vec<ExportConfig> = load_all_export_config().await?;
    let export_config_map: HashMap<String, Vec<ExportConfig>> = export_config_list.iter()
        .fold(HashMap::new(),|mut map, export_config|{
            map.entry(export_config.name.clone())
                .and_modify(|list|list.push(export_config.clone()))
                .or_insert_with(||vec![export_config.clone()]);
            map
        });
    let store = get_export_store().unwrap();
    let mut map = store.inner.write().unwrap();
    for (plugin_name,export_config_list) in export_config_map.iter() {
        // initialize_protocol(protocol_name.clone(), sender.clone(), device_list.to_vec()).await?;
        let value = map.get(plugin_name);
       let export_config= value.unwrap().write();
        export_config.unwrap().initialize(export_config_list.clone())?;
        //初始化数据
    }
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
use sqlx::{Pool, Sqlite};
use crate::handler::plugin_handler::load_all_protocol;
use crate::config::error::Result;
use crate::config::cache::get_protocol_store;
use crate::config::device_shadow::handler_event;
use crate::handler::device_handler::load_all_device_details;

//初始化协议
pub(crate) async fn init_protocol(pool: Pool<Sqlite>)->Result<()> {
   let protocols= load_all_protocol(pool.clone())
       .await?;
  let store=  get_protocol_store().unwrap();
    let (sender, receiver) = std::sync::mpsc::channel();
    let device_map=load_all_device_details(pool).await;
    let device_map=match device_map {
        Ok(map) => map,
        Err(err) => {
            tracing::error!("启动获取设备失败:{:?}",err);
            panic!("启动获取设备失败")
        }
    };
    for protocol_config in protocols.iter() {
        //如果是系统插件,直接初始化
       let device_list= device_map.get(&protocol_config.id).map(|list|list.to_vec())
           .unwrap_or(vec![]);
        store.load_protocol(protocol_config,sender.clone(),device_list).await?;
    }
    tokio::spawn(async move{
        for point_event in receiver {
            handler_event(point_event).await;
        }
    });
    Ok(())
}
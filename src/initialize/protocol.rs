use sqlx::{Pool, Sqlite};
use crate::config::error::Result;
use crate::config::cache::{get_protocol_store, initialize_protocol};
use crate::config::device_shadow::handler_event;
use crate::handler::device_handler::load_all_device_details;

//初始化协议
pub(crate) async fn init_protocol(pool: Pool<Sqlite>) -> Result<()> {
    register_all_protocol().await;
    let (sender, mut receiver) = tokio::sync::mpsc::channel(10);
    let device_map = load_all_device_details(pool).await;
    let device_map = match device_map {
        Ok(map) => map,
        Err(err) => {
            tracing::error!("启动获取设备失败:{:?}",err);
            panic!("启动获取设备失败")
        }
    };
    for (protocol_name, device_list) in device_map.iter() {
        match initialize_protocol(protocol_name.clone(), sender.clone(), device_list.to_vec()).await {
            Ok(_) => {}
            Err(err) => {
                tracing::error!("初始化协议失败:{:?}",err);
            }
        }
    }
    tokio::spawn(async move {
        while let Some(point_event) = receiver.recv().await {
            handler_event(point_event).await;
        }
    });
    Ok(())
}

pub async fn register_all_protocol() {
    let store = get_protocol_store().unwrap();
    #[cfg(feature = "modbus-tcp")]
    {
        protocol_modbus_tcp::register_protocol(store).await;
    }
}
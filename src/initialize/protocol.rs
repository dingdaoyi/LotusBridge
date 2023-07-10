use sqlx::{Pool, Sqlite};
use crate::config::error::Result;
use crate::config::cache::{get_protocol_store, initialize_protocol};
use crate::config::device_shadow::handler_event;
use crate::handler::device_handler::load_all_device_details;

//初始化协议
pub(crate) async fn init_protocol(pool: Pool<Sqlite>) -> Result<()> {
    register_all_protocol().await;
    let (sender, receiver) = std::sync::mpsc::channel();
    let device_map = load_all_device_details(pool).await;
    let device_map = match device_map {
        Ok(map) => map,
        Err(err) => {
            tracing::error!("启动获取设备失败:{:?}",err);
            panic!("启动获取设备失败")
        }
    };
    for (protocol_name, device_list) in device_map.iter() {
        initialize_protocol(protocol_name.clone(), sender.clone(), device_list.to_vec()).await?;
    }
    tokio::spawn(async move {
        for point_event in receiver {
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
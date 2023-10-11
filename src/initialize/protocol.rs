use crate::config::cache::{get_protocol, get_protocol_store, initialize_protocol};
use crate::config::device_shadow::handler_event;
use crate::config::error::Result;
use crate::handler::device_handler::{device_details, load_all_device_details};
use sqlx::{Pool, Sqlite};
use protocol_core::ProtocolState;

//初始化协议
pub(crate) async fn init_protocol(pool: Pool<Sqlite>) -> Result<()> {
    register_all_protocol().await;
    let device_map = load_all_device_details(pool).await;
    let (sender, mut receiver) = tokio::sync::mpsc::channel(10);
    let device_map = match device_map {
        Ok(map) => map,
        Err(err) => {
            tracing::error!("启动获取设备失败:{:?}", err);
            panic!("启动获取设备失败")
        }
    };
    for (protocol_name, device_list) in device_map.iter() {
        match initialize_protocol(protocol_name.clone(), sender.clone(), device_list.to_vec()).await
        {
            Ok(_) => {}
            Err(err) => {
                tracing::error!("初始化协议失败:{:?}", err);
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

/**
 * 添加设备到协议中
 */
pub async fn add_device_to_protocol(
    protocol_name: String,
    device_id: i32,
) -> Result<()> {
    let device = device_details(device_id).await?;

    let (sender, mut receiver) = tokio::sync::mpsc::channel(1);
    let protocol = get_protocol(&protocol_name).await;
    let mut protocol = protocol.lock().await;
    match protocol.get_state() {
        ProtocolState::NoInitialized => {
            match initialize_protocol(protocol_name.clone(), sender.clone(), vec![device]).await
            {
                Ok(_) => {}
                Err(err) => {
                    tracing::error!("初始化协议失败:{:?}", err);
                }
            }
        }
        ProtocolState::Running => {
            protocol.add_device(device).unwrap();
        }
        ProtocolState::Closed => {}
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
        protocol_modbus::register_modbus_tcp(store).await;
    }
    #[cfg(feature = "modbus-rtu")]
    {
        protocol_modbus::register_modbus_rtu(store).await;
    }
}

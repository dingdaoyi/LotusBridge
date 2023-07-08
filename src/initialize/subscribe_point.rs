// use sqlx::SqlitePool;
// use crate::config::cache::get_protocol_store;
// use crate::config::device_shadow::handler_event;
// use crate::handler::device_handler::load_all_device_details;
//
// pub async fn init_subscribe_point(pool: SqlitePool) {
//     let state= get_protocol_store().unwrap();
//     let (sender, receiver) = std::sync::mpsc::channel();
//     let device_map=load_all_device_details(pool).await;
//     let device_map=match device_map {
//         Ok(map) => map,
//         Err(err) => {
//             tracing::error!("启动获取设备失败:{:?}",err);
//             panic!("启动获取设备失败")
//         }
//     };
//     let res=state.init_protocol(sender,device_map);
//     match res {
//         Ok(_) => {}
//         Err(err) => {
//             tracing::error!("初始化错误:{:?}",err);
//         }
//     };
//     tokio::spawn(async move{
//         for point_event in receiver {
//             handler_event(point_event).await;
//         }
//     });
// }

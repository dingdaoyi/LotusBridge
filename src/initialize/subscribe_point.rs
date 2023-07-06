use crate::config::cache::get_protocol_store;
use crate::config::device_shadow::handler_event;

pub async fn init_subscribe_point() {
    let state= get_protocol_store().unwrap();
    let (sender, receiver) = std::sync::mpsc::channel();
    let res=state.init_protocol(sender);
    match res {
        Ok(_) => {}
        Err(err) => {
            tracing::error!("初始化错误:{:?}",err);
        }
    };
    tokio::spawn(async move{
        for point_event in receiver {
            handler_event(point_event).await;
        }
    });
}

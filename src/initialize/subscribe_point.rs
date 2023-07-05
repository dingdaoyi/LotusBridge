use futures::StreamExt;
use protocol_core::event_bus::{get_pubsub_model, PointEvent, set_pharos_pub_sub_model};
use protocol_core::PharosPubSubModel;
use pharos::{ObserveConfig, Channel};
use crate::config::cache::get_protocol_store;

pub async fn init_subscribe_point() {
    let mut pub_sub_model: PharosPubSubModel = PharosPubSubModel::new();
    set_pharos_pub_sub_model(pub_sub_model.clone()).unwrap();
    let ob = pub_sub_model.pharos.observe_shared(ObserveConfig::from(Channel::Bounded(2)));
    let mut events = ob.await.expect("aaaa");
    let state= get_protocol_store().unwrap();
    let res=state.init_protocol();
    match res {
        Ok(_) => {}
        Err(err) => {
            tracing::error!("初始化错误:{:?}",err);

        }
    }
    loop {
        match events.next().await {
            None => {}
            Some(event) => handler_event(event)
        }
    }

}

// 处理上报逻辑
fn handler_event(event: PointEvent) {
    tracing::info!("点位:{},值:{:?}",event.point_id,event.value)
}
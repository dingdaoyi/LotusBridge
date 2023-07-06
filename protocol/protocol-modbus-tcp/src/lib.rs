use std::sync::{Arc, mpsc, Mutex, RwLock};
use std::sync::mpsc::SendError;
use protocol_core::{Value, Protocol, Device};
use protocol_core::Value::Integer;
use yanbing_proc_macro::CreateProtocol;
use tokio::time::{sleep, Duration};
use protocol_core::event_bus::PointEvent;

#[derive(CreateProtocol)]
pub struct ModbusTcpProtocol {
    device_list: Vec<Device>,
    sender: Option<mpsc::Sender<PointEvent>>,

}

impl ModbusTcpProtocol {
    pub async fn schedule_event(&self) {
        // 获取 PharosPubSubModel 实例
        //     let pub_sub_model = get_pubsub_model().unwrap().lock().unwrap(); 为啥这么写报错

        // 创建一个定时器，每隔 1 秒发送一个事件
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        tracing::info!("开始发送数据...");
        println!("开始发送数据...");
        let sender = self.sender.clone().unwrap();
        loop {

            // 等待定时器触发
            interval.tick().await;
            // 创建要发送的事件
            let event = PointEvent {
                point_id: 1,
                value: Value::Integer(42),
            };
            let res = sender.send(event);
            match res {
                Ok(_) => {
                    tracing::info!("发送事件...")
                }
                Err(e) => {
                    tracing::info!("发送事件失败...{:?}",e)
                }
            };
        }
    }
}

unsafe impl Send for ModbusTcpProtocol {}

unsafe impl Sync for ModbusTcpProtocol {}

impl Default for ModbusTcpProtocol {
    fn default() -> Self {
        ModbusTcpProtocol {
            device_list: vec![],
            sender: None,
        }
    }
}

impl Protocol for ModbusTcpProtocol {
    fn read_point(&self, _point_id: i64) -> Result<Value, String> {
        Ok(Value::Integer(10))
    }

    fn write_point(&self, _point_id: i64, _value: Value) -> Result<Value, String> {
        Ok(Value::Integer(10))
    }

    fn initialize(&mut self, _device_list: Vec<Device>, sender: mpsc::Sender<PointEvent>) -> Result<(), String> {
        self.sender = Some(sender);
        self.device_list = _device_list;
        let rs = tokio::runtime::Runtime::new().unwrap();
        rs.block_on(async {
            self.schedule_event().await;
        });
        Ok(())
    }


    fn stop(&self, _force: bool) -> Result<(), String> {
        todo!()
    }

    fn add_device(&self, _device: protocol_core::Device) -> Result<(), String> {
        todo!()
    }

    fn remove_device(&self, _device_id: i64) -> Result<(), String> {
        todo!()
    }

    fn update_device(&self, _device: protocol_core::Device) -> Result<(), String> {
        todo!()
    }
}

#[cfg(test)]
mod testing {
    #[test]
    fn test() {}
}

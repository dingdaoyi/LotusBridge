use std::sync::mpsc;
use protocol_core::{Value, Protocol, Device};

use yanbing_proc_macro::CreateProtocol;
use protocol_core::event_bus::PointEvent;
use std::time::Duration;
#[derive(CreateProtocol)]
pub struct ModbusTcpProtocol {
    device_list: Vec<Device>,
    sender: Option<mpsc::Sender<PointEvent>>,
}

impl ModbusTcpProtocol {
    pub fn schedule_event(&self) {
        let sender = self.sender.clone().unwrap();
        tracing::info!("开始发送数据...");
        println!("开始发送数据...");
        let rs = tokio::runtime::Runtime::new().unwrap();

        rs.block_on(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(10)).await;
                let event = PointEvent {
                    point_id: 2,
                    value: Value::Integer(42),
                };
                let res = sender.send(event);
                match res {
                    Ok(_) => {
                        tracing::info!("发送事件...");
                    }
                    Err(e) => {
                        tracing::info!("发送事件失败...{:?}", e);
                    }
                };
            }
        });
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
    fn read_point(&self, _point_id: i32) -> Result<Value, String> {
        Ok(Value::Integer(10))
    }

    fn write_point(&self, _point_id: i32, _value: Value) -> Result<Value, String> {
        Ok(Value::Integer(10))
    }

    fn initialize(&mut self, _device_list: Vec<Device>, sender: mpsc::Sender<PointEvent>) -> Result<(), String> {
        self.sender = Some(sender);
        self.device_list = _device_list;
        self.schedule_event();
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

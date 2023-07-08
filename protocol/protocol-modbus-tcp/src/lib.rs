use std::collections::HashMap;
use std::convert::Into;
use std::string::ToString;
use std::sync::{Arc, mpsc, Mutex};
use protocol_core::{Value, Protocol, Device, ReadPointRequest, WriterPointRequest};

use yanbing_proc_macro::CreateProtocol;
use protocol_core::event_bus::PointEvent;
use std::time::Duration;
use modbus::Client;

const MODBUS_TCP_ADDRESS: &'static str = "address";
const MODBUS_TCP_PORT: &'static str = "port";
const MODBUS_TCP_DEFAULT_PORT: u16 = 502;
const MODBUS_TCP_DEFAULT_HOST: &'static str = "127.0.0.1";

type ModbusClient = modbus::tcp::Transport;

#[derive(CreateProtocol)]
pub struct ModbusTcpProtocol {
    device_list: Vec<Device>,
    sender: Option<mpsc::Sender<PointEvent>>,
    modbus_client: HashMap<i32, Arc<Mutex<ModbusClient>>>,
}

impl ModbusTcpProtocol {
    //TODO 当前问题,需要一直阻塞在线程里面,先实现功能后续再看问题
    pub fn schedule_event(&self) {
        let sender = self.sender.clone().unwrap();
        tracing::info!("开始发送数据...");
        println!("开始发送数据...");
        let rs = tokio::runtime::Runtime::new().unwrap();
        rs.block_on(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1000)).await;
                let event = PointEvent {
                    point_id: 100,
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

    fn init_modbus(&mut self) {
        for Device { id, custom_data, .. } in self.device_list.iter() {
            let mut config = modbus::tcp::Config::default();
            let mut address = MODBUS_TCP_DEFAULT_HOST.to_string();
            if let Some(custom_address) = custom_data.get(MODBUS_TCP_ADDRESS) {
                address = custom_address.clone();
            }
            let port_str: Option<&String> = custom_data.get(MODBUS_TCP_PORT);
            let port: Option<u16> = port_str.and_then(|s| s.parse().ok());
            config.tcp_port = port.unwrap_or(MODBUS_TCP_DEFAULT_PORT);
            let client = ModbusClient::new_with_cfg(&address, config).unwrap();
            self.modbus_client.insert(*id, Arc::new(Mutex::new(client)));
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
            modbus_client: HashMap::new(),
        }
    }
}

impl Protocol for ModbusTcpProtocol {
    fn read_point(&self, request: ReadPointRequest) -> Result<Value, String> {
        let res = self.modbus_client.get(&request.device_id).unwrap();
        let mut client = res.lock().unwrap();

        let Address{address,..} = parse_address(request.address.as_str()).unwrap_or(Address{
            device_id: 1,
            function: 3,
            address: 0,
        });
        //TODO 需要将address区分,以及不同类型的返回值转化
        let res: modbus::Result<Vec<u16>> = client.read_input_registers(address, 1);
        match res {
            Ok(data) => {
                if let Some(value) = data.first().cloned() {
                    // 或者根据实际需要选择适当的数字类型
                    let value_as_number: i32 = value.into();
                    Ok(Value::Integer(value_as_number))
                } else {
                    Err("No data received".to_string())
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }

    fn write_point(&self, _request: WriterPointRequest) -> Result<Value, String> {
        Ok(Value::Integer(10))
    }

    fn initialize(&mut self, device_list: Vec<Device>, sender: mpsc::Sender<PointEvent>) -> Result<(), String> {
        println!("协议包含数据:{:?}", device_list);
        self.sender = Some(sender);
        self.device_list = device_list;
        self.init_modbus();
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
struct Address {
    device_id: u16,
    function: u16,
    address: u16,
}

fn parse_address(address: &str) -> Option<Address> {
    let parts: Vec<&str> = address.split('!').collect();
    if parts.len() == 2 {
        let device_id_str = parts[0];
        let function_address_str = parts[1];
        let (function_str, address_str) = function_address_str.split_at(1);

        if let Ok(device_id) = device_id_str.parse::<u16>() {
            if let Ok(address) = address_str.parse::<u16>() {
                if let Ok(function) = function_str.parse::<u16>() {
                    return Some(Address {
                        device_id,
                        function,
                        address,
                    });
                }
            }
        }
    }
    None
}


#[cfg(test)]
mod testing {
    #[test]
    fn test() {}
}

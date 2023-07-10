use std::collections::HashMap;
use std::convert::Into;
use std::string::ToString;
use std::sync::{Arc, mpsc, Mutex};
use protocol_core::{Value, Protocol, Device, ReadPointRequest, WriterPointRequest};
use protocol_core::event_bus::PointEvent;
use modbus::{Client, Config, Transport};
use protocol_core::protocol_store::ProtocolStore;

const MODBUS_TCP_ADDRESS: &'static str = "address";
const PROTOCOL_NAME: &'static str = "modbus-tcp";
const MODBUS_TCP_PORT: &'static str = "port";
const MODBUS_TCP_DEFAULT_PORT: u16 = 502;
const MODBUS_TCP_DEFAULT_HOST: &'static str = "127.0.0.1";

type ModbusClient = Transport;

pub struct ModbusTcpProtocol {
    device_list: Vec<Device>,
    sender: Option<mpsc::Sender<PointEvent>>,
    modbus_client: HashMap<i32, Arc<Mutex<ModbusClient>>>,
}

impl ModbusTcpProtocol {

    fn connect_modbus_slave(&mut self, device_id: i32, address: &str, config: Config) -> Result<(), String> {
        let client = ModbusClient::new_with_cfg(address, config)
            .map_err(|err| err.to_string())?;
        self.modbus_client.insert(device_id, Arc::new(Mutex::new(client)));
        Ok(())
    }
    fn init_modbus(&mut self) {
        let device_list = std::mem::take(&mut self.device_list); // 获取所有权
        for Device { id, custom_data, .. } in device_list {
            let mut config = modbus::tcp::Config::default();
            let mut address = MODBUS_TCP_DEFAULT_HOST.to_string();
            if let Some(custom_address) = custom_data.get(MODBUS_TCP_ADDRESS) {
                address = custom_address.clone();
            }
            let port_str: Option<&String> = custom_data.get(MODBUS_TCP_PORT);
            let port: Option<u16> = port_str.and_then(|s| s.parse().ok());
            config.tcp_port = port.unwrap_or(MODBUS_TCP_DEFAULT_PORT);

            if  let Err(err) = self.connect_modbus_slave(id, &address, config) {
                println!("错误链接,请检查设备是否正常:{}", err);
                // tokio::time::sleep(Duration::from_secs(60)).await;
            }
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
        let res = self.modbus_client
            .get(&request.device_id)
            .ok_or("modbus slave 不存在,请检查协议配置".to_string())?;

        let mut client = res.lock().unwrap();

        let Address { address, .. } = parse_address(request.address.as_str()).unwrap_or(Address {
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
            // continuation...
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
pub async fn register_protocol(store: &ProtocolStore) {
    let protocol=ModbusTcpProtocol::default();
    store.register_protocol(PROTOCOL_NAME.to_string(),protocol);
}

#[cfg(test)]
mod testing {
    #[test]
    fn test() {}
}

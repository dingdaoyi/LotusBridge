use std::collections::HashMap;
use std::string::ToString;
use std::sync::{Arc, Mutex};
use tokio_modbus::client::sync::Context;
use tokio_modbus::prelude::sync::rtu;
use tokio_modbus::prelude::SyncReader;
use protocol_core::{Value, Protocol, Device, ReadPointRequest, WriterPointRequest};
use protocol_core::event_bus::PointEvent;
use tokio_modbus::Slave;

type ModbusClient = Context;

pub struct ModbusRtuProtocol {
    device_list: Vec<Device>,
    sender: Option<tokio::sync::mpsc::Sender<PointEvent>>,
    modbus_client: HashMap<i32, Arc<Mutex<ModbusClient>>>,
}

impl ModbusRtuProtocol {

    pub(crate) fn init_modbus(&mut self) -> Result<(), String> {
        for device in &self.device_list {
            let custom_data = &device.custom_data;
                let tty_path = custom_data.get("tty_path").map(|e|e.to_string()).unwrap_or("/dev/ttyUSB0".into());
                let slave_id = custom_data.get("slave_id").map(|v| v.parse().unwrap_or(1)).unwrap_or(1);
                let port = custom_data.get("port").map(|v| v.parse().unwrap_or(19200)).unwrap_or(19200);
                let slave = Slave(slave_id);
                let builder = tokio_serial::new(tty_path, port);
                let ctx = rtu::connect_slave(&builder, slave).map_err(|e| e.to_string())?;
                self.modbus_client.insert(device.id, Arc::new(Mutex::new(ctx)));
        }
        Ok(())
    }
}


impl ModbusRtuProtocol {

}


unsafe impl Send for ModbusRtuProtocol {}

unsafe impl Sync for ModbusRtuProtocol {}

impl Default for ModbusRtuProtocol {
    fn default() -> Self {
        ModbusRtuProtocol {
            device_list: vec![],
            sender: None,
            modbus_client: HashMap::new(),
        }
    }
}

impl Protocol for ModbusRtuProtocol {

    fn read_point(&self, request: ReadPointRequest) -> Result<Value, String> {
        let res = self.modbus_client
            .get(&request.device_id)
            .ok_or("设备未找到")?;

        let mut client = res.lock().unwrap();

        let Address { address,function, .. } = parse_address(request.address.as_str()).unwrap_or(Address {
            device_id: 1,
            function: 3,
            address: 0,
        });
        match function {
            0 | 1 => {
                let result = match function {
                    0 => client.read_coils(address, 1),
                    1 => client.read_discrete_inputs(address, 1),
                    _ => unreachable!(), // 0 or 1
                };

                result.map(|coil| {
                    Value::Boolean(coil.first().map(|value| *value ).unwrap_or(false))
                }).map_err(|e| e.to_string())
            }
            3 | 4 => {
                let result = match function {
                    3 => client.read_input_registers(address, 1),
                    4 => client.read_holding_registers(address, 1),
                    _ => unreachable!(), // 3 or 4
                };

                result.map(|words| {
                    Value::Integer(words.first().map(|value| *value as i32).unwrap_or(0))
                }).map_err(|e| e.to_string())
            }
            _ => Err("未知类型".to_string()),
        }
    }

    fn write_point(&self, request: WriterPointRequest) -> Result<Value, String> {
        unimplemented!()
    }
    fn initialize(&mut self, device_list: Vec<Device>, sender: tokio::sync::mpsc::Sender<PointEvent>) -> Result<(), String> {
        self.device_list = device_list;
        self.sender = Some(sender);
        self.init_modbus()?;
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
struct Address {
    device_id: u16,
    function: u16,
    address: u16,
}

#[cfg(test)]
mod testing {
    #[test]
    fn test() {}
}

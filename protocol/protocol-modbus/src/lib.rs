use std::collections::HashMap;
use std::io::Error;
use std::string::ToString;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::runtime::Runtime;
use tokio::sync::{Mutex, RwLock};
use protocol_core::{Value, Protocol, Device, ReadPointRequest, WriterPointRequest, ProtocolError};
use protocol_core::event_bus::PointEvent;
use tokio_modbus::client::{Context, Reader, Writer, tcp, rtu};
use tokio_modbus::Slave;
use tokio_serial::SerialStream;
use protocol_core::protocol_store::ProtocolStore;
use crate::ModbusType::{RTU, TCP};

const MODBUS_TCP_ADDRESS: &'static str = "address";
const TCP_PROTOCOL_NAME: &'static str = "modbus-tcp";
const RTU_PROTOCOL_NAME: &'static str = "modbus-rtu";
const MODBUS_TCP_PORT: &'static str = "port";
const MODBUS_SLAVE_ID: &'static str = "slave_id";
const MODBUS_TCP_DEFAULT_PORT: u16 = 502;
const MODBUS_TCP_DEFAULT_HOST: &'static str = "127.0.0.1";

type ModbusClient = Context;

#[async_trait]
pub trait ModbusInitializer {
    async fn init_modbus(&self, protocol: & mut ModbusTcpProtocol) ;
}


pub struct ModbusTcpProtocol {
    modbus_type: ModbusType,
    device_list: Vec<Device>,
    sender: Option<tokio::sync::mpsc::Sender<PointEvent>>,
    modbus_client: HashMap<i32, Arc<Mutex<ModbusClient>>>,
}

impl ModbusTcpProtocol {

    fn new(init_modbus:ModbusType) -> Self {
        ModbusTcpProtocol {
            modbus_type: init_modbus,
            device_list: vec![],
            sender: None,
            modbus_client: HashMap::new(),
        }
    }
    #[cfg(feature = "modbus-tcp")]
    async fn init_tcp_modbus(&mut self,) {
        for device in &self.device_list {
            let custom_data = &device.custom_data;
            let address = custom_data.get(MODBUS_TCP_ADDRESS).map(|e| e.to_string()).unwrap_or(MODBUS_TCP_DEFAULT_HOST.into());
            let port = custom_data.get(MODBUS_TCP_PORT).map(|v| v.parse().unwrap_or(MODBUS_TCP_DEFAULT_PORT)).unwrap_or(MODBUS_TCP_DEFAULT_PORT);
            let slave_id = custom_data.get(MODBUS_SLAVE_ID).map(|v| v.parse().unwrap_or(1)).unwrap_or(1);
            let slave = Slave(slave_id);
            let socket_addr = format!("{}:{}", address, port).parse().unwrap();
            let ctx = tcp::connect_slave(socket_addr, slave).await;
            match ctx {
                Ok(ctx) => {
                    self.modbus_client
                        .insert(device.id, Arc::new(Mutex::new(ctx)));
                }
                Err(_) => {}
            }

        }
    }

    #[cfg(feature = "modbus-rtu")]
    async fn init_rtu_modbus(&mut self){
        for device in &self.device_list {
            let custom_data = &device.custom_data;
            let tty_path = custom_data.get("tty_path").map(|e|e.to_string()).unwrap_or("/dev/ttyUSB0".into());
            let slave_id = custom_data.get("slave_id").map(|v| v.parse().unwrap_or(1)).unwrap_or(1);
            let port = custom_data.get("port").map(|v| v.parse().unwrap_or(19200)).unwrap_or(19200);
            let slave = Slave(slave_id);

            let builder = tokio_serial::new(&tty_path, 19200);
            match SerialStream::open(&builder) {
                Ok(port) => {
                    let ctx = rtu::attach_slave(port, slave);
                    self.modbus_client.insert(device.id, Arc::new(Mutex::new(ctx)));
                }
                Err(msg) => {
                    tracing::error!("Failed to open \"{}\". Error: {}", &tty_path, msg);
                }
            }

        }
    }
}


unsafe impl Send for ModbusTcpProtocol {}

unsafe impl Sync for ModbusTcpProtocol {}


#[derive(Debug)]
pub enum ReadPointError {
    ModbusSlaveNotFound,
    UnknownFunction,
    NoDataReceived,
}
impl std::fmt::Display for ReadPointError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       let message= match self {
            ReadPointError::ModbusSlaveNotFound => "modbus slave不存在，请检查协议配置",
            ReadPointError::UnknownFunction => "未知功能",
            ReadPointError::NoDataReceived => "未收到数据",
        };
        write!(f, "{}",message)
    }
}

impl Into<ProtocolError> for ReadPointError{

    fn into(self) -> ProtocolError {
        ProtocolError::from(self.to_string())
    }
}

#[async_trait]
impl Protocol for ModbusTcpProtocol {
    async fn read_point(&self, request: ReadPointRequest) -> Result<Value, ProtocolError> {
        let res = self.modbus_client
            .get(&request.device_id)
            .ok_or(ReadPointError::ModbusSlaveNotFound.to_string())?;

        let mut client = res.lock().await;

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
                    _ => unreachable!(),
                };

                result.await.map(|coil| {
                    Value::Boolean(coil.first().map(|value| *value ).unwrap_or(false))
                }).map_err(|e| ProtocolError::from(e))
            }
            3 | 4 => {
                let result = match function {
                    3 => client.read_input_registers(address, 1),
                    4 => client.read_holding_registers(address, 1),
                    _ => unreachable!(),
                };

                result.await.map(|words| {
                    Value::Integer(words.first().map(|value| *value as i32).unwrap_or(0))
                }).map_err(|e| ProtocolError::from(e))
            }
            _ => Err("未知类型".into()),
        }
    }

    async fn write_point(&self, request: WriterPointRequest) -> Result<Value, ProtocolError> {
        let res = self.modbus_client
            .get(&request.device_id)
            .ok_or(ReadPointError::ModbusSlaveNotFound.to_string())?;

        let mut client = res.lock().await;

        let Address { address,function, device_id: _device_id } = parse_address(request.address.as_str()).unwrap_or(Address {
            device_id: 1,
            function: 3,
            address: 0,
        });

        match function {
            0=>{
                if let Value::Boolean(value) = request.value {
                    client.write_single_coil(address, value)
                        .await
                        .map_err(|e| ProtocolError::from(e))?;
                }
            },
            4=> {
                if let Value::Integer(value) = request.value {
                    client.write_single_register(address, value as u16)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            },
            _ =>return Err(ReadPointError::UnknownFunction.into())
        };
        Ok(request.value)
    }
    async fn initialize(&mut self, device_list: Vec<Device>, sender: tokio::sync::mpsc::Sender<PointEvent>) -> Result<(), ProtocolError> {
        println!("协议包含数据:{:?}", device_list);
        self.sender = Some(sender);
        self.device_list = device_list;
        match &self.modbus_type {

            RTU => {
                #[cfg(feature = "modbus-rtu")]
                self.init_rtu_modbus().await;
            }

            TCP => {
                #[cfg(feature = "modbus-tcp")]
                self.init_tcp_modbus().await;
            }
        }
        Ok(())
    }

    fn stop(&self, _force: bool) -> Result<(), ProtocolError> {
        todo!()
    }


    fn add_device(&self, _device: protocol_core::Device) -> Result<(), ProtocolError> {
        todo!()
    }

    fn remove_device(&self, _device_id: i64) -> Result<(), ProtocolError> {
        todo!()
    }

    fn update_device(&self, _device: protocol_core::Device) -> Result<(), ProtocolError> {
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
#[cfg(feature = "modbus-tcp")]
pub async fn register_modbus_tcp(store: &ProtocolStore) {
    let protocol=ModbusTcpProtocol::new(TCP_PROTOCOL_NAME.into());
    store.register_protocol(TCP_PROTOCOL_NAME.to_string(),protocol);
}

#[cfg(feature = "modbus-rtu")]
pub async fn register_modbus_rtu(store: &ProtocolStore) {
    let protocol=ModbusTcpProtocol::new(RTU_PROTOCOL_NAME.into());
    store.register_protocol(RTU_PROTOCOL_NAME.to_string(),protocol);
}


enum ModbusType{
    RTU,
    TCP,
}

/// String 转 StructInitializer
impl ToString for ModbusType {
    fn to_string(&self) -> String {
        match self {
            RTU => RTU_PROTOCOL_NAME.to_string(),
            TCP=> TCP_PROTOCOL_NAME.to_string(),
        }
    }
}
impl From<&str> for ModbusType{
    fn from(value: &str) -> Self {
       match value {
           RTU_PROTOCOL_NAME=>RTU,
              TCP_PROTOCOL_NAME=>TCP,
           _ => unreachable!(),
       }
    }
}

#[cfg(test)]
mod testing {
    #[test]
    fn test() {}
}

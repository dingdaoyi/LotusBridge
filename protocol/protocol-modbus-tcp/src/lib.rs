use protocol_core::{Value, Protocol};
use yanbing_proc_macro::CreateProtocol;

#[derive(Default,CreateProtocol)]
pub struct ModbusTcpProtocol;

impl Protocol for ModbusTcpProtocol {
    fn read_point(&self, _point_id: i64) -> Result<Value, String> {
        Ok(Value::Integer(10))
    }

    fn write_point(&self, _point_id: i64, _value: Value) -> Result<Value, String> {
        Ok(Value::Integer(10))
    }

    fn initialize(&self) -> Result<(), String> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use crate::create_protocol;
    

    #[test]
     fn test_modbus_tcp_protocol_create() {
        let _protocol =unsafe{ create_protocol()};
    }
}


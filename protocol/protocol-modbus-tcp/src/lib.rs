use protocol_core::{DecoderResult, DecoderDataItem, Value, Protocol};
use yanbing_proc_macro::CreateProtocol;

#[derive(Default,CreateProtocol)]
pub struct ModbusTcpProtocol;

impl Protocol for ModbusTcpProtocol {
    fn process_data(&self, _data: &[u8]) -> Result<DecoderResult, String> {
        println!("process_data");
        let decoder_data_item = DecoderDataItem {
            // 设置 DecoderDataItem 的字段值
            identifier: "example".to_string(),
            unit_address: "1234".to_string(),
            unit_type: Some(1),
            unit_type_name: Some("ExampleUnit".to_string()),
            system_type: Some(2),
            unit_description: Some("Example description".to_string()),
            value: Some(Value::Integer(42)),
            system_address: Some(5678),
            child_device_code: Some("ABC123".to_string()),
            acquisition_time: None,
        };

        let decoder_result = DecoderResult {
            decoder_data_item_list: vec![decoder_data_item],
            ack_message: None,
            message_id: None,
            driver_service_name: None,
            driver_name: None,
            imsi: None,
            ack_topic: None,
            iccid: None,
            trd_device_id: None,
            source_address: None,
        };
        println!("post process_data");
        Ok(decoder_result)
    }
}


#[cfg(test)]
mod tests {
    use crate::create_protocol;
    use super::*;

    #[test]
     fn test_modbus_tcp_protocol_create() {
        let protocol =unsafe{ create_protocol()};
    }
}


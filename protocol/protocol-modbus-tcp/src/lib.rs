use protocol_core::{DecoderResult, DecoderDataItem, Value, Protocol};

#[derive(Default)]
pub struct ModbusTcpProtocol{

}

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


#[no_mangle]
pub extern "C" fn create_protocol() -> *mut dyn Protocol {
   let obj= ModbusTcpProtocol::default();
    let boxed: Box<dyn Protocol> = Box::new(obj);
    Box::into_raw(boxed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modbus_tcp_protocol_process_data() {
        // 准备测试数据
        let data: Vec<u8> = vec![1, 3, 5, 6];

        // 创建 ModbusTcpProtocol 对象
        let protocol = ModbusTcpProtocol;

        // 调用 process_data 方法
        let result = protocol.process_data(&data);

        // 检查返回结果
        assert!(result.is_ok());
        let decoder_result = result.unwrap();
        assert_eq!(decoder_result.decoder_data_item_list.len(), 1);

        // 检查 DecoderDataItem 的字段值
        let decoder_data_item = &decoder_result.decoder_data_item_list[0];
        assert_eq!(decoder_data_item.identifier, "example");
        assert_eq!(decoder_data_item.unit_address, "1234");
        assert_eq!(decoder_data_item.unit_type, Some(1));
        assert_eq!(decoder_data_item.unit_type_name, Some("ExampleUnit".to_string()));
        assert_eq!(decoder_data_item.system_type, Some(2));
        assert_eq!(decoder_data_item.unit_description, Some("Example description".to_string()));
        assert_eq!(decoder_data_item.system_address, Some(5678));
        assert_eq!(decoder_data_item.child_device_code, Some("ABC123".to_string()));
        assert_eq!(decoder_data_item.acquisition_time, None);
        // assert_eq!(decoder_data_item.struct_value_map, None);

        // 检查 value 字段
        match decoder_data_item.value {
            Some(Value::Integer(value)) => {
                assert_eq!(value, 42);
            }
            _ => panic!("Unexpected value type"),
        }
    }
}


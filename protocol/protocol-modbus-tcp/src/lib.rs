use protocol_core::{DecoderResult, DecoderDataItem, Value, Protocol};

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

#[no_mangle]
pub extern "C" fn process_data(
    protocol: *mut ModbusTcpProtocol,
    data: *const u8,
    data_len: usize,
    result: *mut *mut DecoderResult,
) -> i32 {
    println!("1231懂啊这");
    // 检查传入的指针是否为空
    println!("protocol.is_null() {}", protocol.is_null());
    println!(" data.is_null() {}", data.is_null());
    if protocol.is_null() || data.is_null()  {
        return -1;
    }
    println!("123331懂啊这");
    // 转换裸指针为安全引用
    let protocol_ref = unsafe { &*protocol };

    // 将数据转换为字节切片
    let data_slice = unsafe { std::slice::from_raw_parts(data, data_len) };
    println!("1231323懂啊这");
    // 调用协议处理数据的方法
    match protocol_ref.process_data(data_slice) {
        Ok(decoder_result) => {
            // 将 DecoderResult 转换为 Box
            let boxed_result = Box::new(decoder_result);

            // 分配内存用于存储结果的裸指针
            let raw_result = Box::into_raw(boxed_result);
            println!("123132懂啊这");
            // 将裸指针存储在 result 中
            unsafe {
                *result = raw_result;
            }
            println!("懂啊这323");
            0 // 返回成功状态码
        }
        Err(_) => {
            0 // 返回失败状态码
        }
    }
}


#[no_mangle]
pub extern "C" fn create_protocol() -> *mut ModbusTcpProtocol {
    Box::into_raw(Box::new(ModbusTcpProtocol))
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
    use std::ptr;

    #[test]
    fn test_process_data() {
        // 创建 ModbusTcpProtocol 对象
        let protocol = Box::new(ModbusTcpProtocol);

        // 构造示例数据
        let data: Vec<u8> = vec![1, 3, 5, 6];

        // 设置结果指针
        let mut result_ptr: *mut DecoderResult = ptr::null_mut();

        // 调用 process_data 函数
        let result = process_data(
            Box::into_raw(protocol),
            data.as_ptr(),
            data.len(),
            &mut result_ptr,
        );

        // 检查返回结果
        assert_eq!(result, 0);
        assert!(!result_ptr.is_null());

        // 将结果指针转换回 DecoderResult 引用
        let decoder_result = unsafe { Box::from_raw(result_ptr) };

        // 检查 DecoderResult 的字段值
        assert_eq!(decoder_result.decoder_data_item_list.len(), 1);
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
    }
}


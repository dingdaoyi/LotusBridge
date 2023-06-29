use axum::extract::State;
use axum::Json;
use sqlx::SqlitePool;
use crate::config::error::{EdgeError, Result};
use protocol_core::{Protocol, DecoderResult};
use libloading::{Library, Symbol};
use std::ptr;

pub async fn test_protocol(State(_pool): State<SqlitePool>) -> Result<Json<DecoderResult>> {
    // 动态加载协议库
    let lib_path = "./target/debug/libprotocol_modbus_tcp.dylib"; // 替换为实际的协议库路径
    let lib = unsafe {
        Library::new(lib_path).expect("Failed to load protocol library")
    };

    // 获取 create_protocol 函数符号
    type CreateProtocolFn = extern "C" fn() -> *mut dyn Protocol;
    let create_protocol: Symbol<CreateProtocolFn> = unsafe {
        lib.get(b"create_protocol").expect("Failed to get symbol")
    };

    // 调用 create_protocol 函数创建协议对象
    let protocol = unsafe { Box::from_raw((create_protocol)()) };
    // pub extern "C" fn process_data(
    //     protocol: *mut ModbusTcpProtocol,
    //     data: *const u8,
    //     data_len: usize,
    //     result: *mut *mut DecoderResult,
    // ) -> i32
    // 获取 process_data 函数符号
    type ProcessDataFn = extern "C" fn(
        protocol: *mut dyn Protocol,
        data: *const u8,
        data_len: usize,
        result: *mut *mut DecoderResult,
    ) -> i32;
    let process_data: Symbol<ProcessDataFn> = unsafe {
        lib.get(b"process_data").expect("Failed to get symbol")
    };

    // 构造示例数据
    let data: Vec<u8> = vec![1, 3, 5, 6];
    let data_ptr = data.as_ptr();
    let data_len = data.len();

// 设置结果指针
    let mut result_ptr: *mut DecoderResult = ptr::null_mut();

// 调用 process_data 函数
    let result = (process_data)(
        Box::into_raw(protocol),
        data_ptr,
        data_len,
        &mut result_ptr,
    );


    if result == 0 {
        // 将结果指针转换回 DecoderResult 引用
        let decoder_result = unsafe { Box::from_raw(result_ptr) };
        Ok(Json(*decoder_result))
    } else {
        Err(EdgeError::Message("aaa".into()))
    }
}

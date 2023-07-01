use axum::extract::State;
use axum::Json;
use sqlx::SqlitePool;
use crate::config::error::{Result};
use protocol_core::{Protocol, DecoderResult};
use libloading::{Library, Symbol};

pub async fn test_protocol(State(_pool): State<SqlitePool>) -> Result<Json<DecoderResult>> {
    // 动态加载协议库
    let lib_path = "./target/debug/libprotocol_modbus_tcp.dylib"; // 替换为实际的协议库路径

    // 获取 create_protocol 函数符号
    type CreateProtocolFn = extern "C" fn() -> *mut dyn Protocol;
    let lib = unsafe {
        Library::new(lib_path).expect("Failed to load protocol library")
    };

    let constructor: Symbol<CreateProtocolFn> = unsafe {
        lib.get(b"create_protocol").expect("Failed to get symbol")
    };
    // 调用该函数，取得 UcenterApp Trait 实例的原始指针
    let boxed_raw = constructor();

    // 通过原始指针构造 Box，至此逻辑重归安全区
    let extend = unsafe{
        Box::from_raw(boxed_raw)
    };
    // 构造示例数据
    let data: Vec<u8> = vec![1, 3, 5, 6];

    let res = extend.process_data(data.as_ref())?;
    Ok(Json(res))
}

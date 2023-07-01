extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CreateProtocol)]
pub fn create_protocol_derive(input: TokenStream) -> TokenStream {
    // 解析输入的 Rust 代码
    let input = parse_macro_input!(input as DeriveInput);

    // 获取结构体的名称
    let struct_name = &input.ident;

    // 生成创建协议对象的代码
    let create_protocol_impl = quote! {
        #[no_mangle]
        // protocol_core::{DecoderResult, DecoderDataItem, Value, Protocol}
        pub extern "C" fn create_protocol() -> *mut dyn protocol_core::Protocol {
            let obj = <#struct_name as Default>::default();
            let boxed: Box<dyn protocol_core::Protocol> = Box::new(obj);
            Box::into_raw(boxed)
        }
    };

    // 将生成的 Rust 代码转换为 TokenStream
    let output = quote! {
        #create_protocol_impl
    };

    output.into()
}

pub mod things;
pub mod device;
pub mod plugin;
pub mod point;
pub mod page;
pub  mod export_config;


use serde::{Serialize, Deserialize};
use std::option::Option;

#[derive(Debug, Serialize, Deserialize)]
pub struct R<T> {
    code: i32,
    success: bool,
    msg: String,
    data: Option<T>,
}

impl<T> R<T> {
    pub fn new(code: i32, success: bool, msg: String, data: Option<T>) -> Self {
        R { code, success, msg, data }
    }

    pub fn success() -> Self {
        R::new(200, true, String::new(), None)
    }

    pub fn success_with_data(data: T) -> Self {
        R::new(200, true, String::new(), Some(data))
    }

    pub fn fail(msg: String) -> Self {
        R::new(500, false, msg, None)
    }

    pub fn bad_request(msg: String) -> Self {
        R::new(400, false, msg, None)
    }

    pub fn fail_with_code(code: i32, msg: String) -> Self {
        R::new(code, false, msg, None)
    }

    pub fn is_success(&self) -> bool {
        self.success
    }

    pub fn is_fail(&self) -> bool {
        !self.success
    }

    pub fn get_data(&self) -> Option<&T> {
        self.data.as_ref()
    }
}

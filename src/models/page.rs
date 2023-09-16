use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PaginationRequest {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_page() -> u32 {
    1 
}

fn default_limit() -> u32 {
    10 
}

#[derive(Debug, Serialize)]
pub struct PaginationResponse<T> {
    pub total: u32,
    pub data: Vec<T>,
}

impl <T>PaginationResponse<T> {
    
    pub fn new(data:Vec<T>,total:u32) -> Self {
         Self{
             total,
             data,
         }
    }
}

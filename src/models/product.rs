use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: i64,
    //产品名称
    pub name: String,
    //产品类型
    #[serde(rename = "productType")]
    pub product_type: ProductType,
}

#[derive(Debug, Deserialize)]
pub struct CreatProduct {
    //产品名称
    pub name: String,
    //产品类型
    #[serde(rename = "productType")]
    pub product_type: ProductType,
}

#[derive(Debug, Serialize, Deserialize,Type)]
pub enum ProductType {
    // 独立设备
    #[serde(rename = "Gateway")]
    Independent,
    // 网关
    #[serde(rename = "Independent")]
    Gateway,
}
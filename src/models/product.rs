use std::error::Error;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::{Decode, Encode, FromRow, Sqlite, Type};
use sqlx::database::{HasArguments, HasValueRef};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::sqlite::SqliteValue;

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

use sqlx::TypeInfo;
use sqlx::encode::IsNull::No;


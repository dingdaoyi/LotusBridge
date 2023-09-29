use crate::config::date_format;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::FromRow;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ProductFunc {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "createTime", with = "date_format")]
    pub create_time: Option<NaiveDateTime>,

    #[serde(rename = "updateTime", with = "date_format")]
    pub update_time: Option<NaiveDateTime>,

    #[serde(rename = "async")]
    #[sqlx(rename = "async")]
    pub is_async: Option<bool>,

    #[serde(rename = "dataType")]
    pub data_type: i32,

    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "eventType")]
    pub event_type: Option<i32>,

    #[serde(rename = "funcStatus")]
    pub func_status: Option<i32>,

    #[serde(rename = "funcType")]
    pub func_type: i32,

    #[serde(rename = "hasRuleEngine")]
    pub has_rule_engine: Option<bool>,

    #[serde(rename = "identifier")]
    pub identifier: String,

    #[serde(rename = "inputParam")]
    pub input_param: Option<Json<Vec<BaseAttrItem>>>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "outputParam")]
    pub output_param: Option<Json<Vec<BaseAttrItem>>>,

    #[serde(rename = "ruleEngine")]
    pub rule_engine: Option<String>,

    #[serde(rename = "specs")]
    pub specs: Json<BaseAttrItem>,

    #[serde(rename = "wrType")]
    #[sqlx(rename = "wr_type")]
    pub is_read_only: Option<bool>,

    #[serde(rename = "productTypeId")]
    pub product_type_id: Option<i64>,

    #[serde(rename = "iconId")]
    pub icon_id: Option<i64>,

    #[serde(rename = "custom")]
    #[sqlx(rename = "custom")]
    pub is_custom: bool,

    #[serde(rename = "optional")]
    #[sqlx(rename = "optional")]
    pub is_optional: bool,

    #[serde(rename = "protectedService")]
    pub protected_service: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseAttrItem {
    pub name: Option<String>,
    pub identifier: Option<String>,
    pub data_type: Option<i32>,
    pub bool0: Option<String>,
    pub bool1: Option<String>,
    pub length: Option<i64>,
    pub unit: Option<String>,
    pub min: Option<f64>,
    pub unit_name: Option<String>,
    pub max: Option<f64>,
    pub step: Option<f64>,
    pub enum_map: Option<HashMap<String, String>>,
}

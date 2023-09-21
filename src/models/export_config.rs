use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Debug)]
pub struct ExportGroupQuery {
    #[serde(rename = "groupIds")]
    pub group_ids: Vec<i32>,
    #[serde(rename = "exportId")]
    pub export_id: i32,
}

#[derive(Deserialize, Debug,Serialize,FromRow)]
pub struct ExportGroup {
    pub id: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "exportId")]
    pub export_id: i32,
}
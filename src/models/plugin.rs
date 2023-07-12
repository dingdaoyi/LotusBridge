use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

// 公共的插件配置,创建使用
#[derive(Debug, Deserialize)]
pub struct CreatePluginConfig {
    //协议名称
    pub name: String,
    pub description: Option<String>,
    pub form_customization: Option<String>,
    // 插件类型
    pub plugin_type: PluginType
}

#[derive(Debug, Serialize,FromRow)]
pub struct PluginConfig {
    pub id: i32,
    pub description: Option<String>,
    pub form_customization: Option<String>,
    //协议名称
    pub name: String,
    // 插件类型
    pub plugin_type: PluginType
}


// 插件类型
#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(untagged)]
pub enum PluginType {
    // 系统插件
    #[serde(rename = "Protocol")]
    Protocol,
    // 自定义插件
    #[serde(rename = "DataOutput")]
    DataOutput,
    // 规则引擎
    #[serde(rename = "RuleEngine")]
    RuleEngine
}

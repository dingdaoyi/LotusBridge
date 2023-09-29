use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

// 公共的插件配置,创建使用
#[derive(Debug, Deserialize)]
pub struct CreatePluginConfig {
    //协议名称
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "formCustomization")]
    pub form_customization: Option<String>,
    // 插件类型
    #[serde(rename = "pluginType")]
    pub plugin_type: PluginType,
}

// 公共的插件配置,创建使用
#[derive(Debug, Deserialize)]
pub struct PluginConfigQuery {
    //协议名称
    pub name: Option<String>,

    #[serde(rename = "pluginType")]
    pub plugin_type: Option<PluginType>,
}

// 插件配置
#[derive(Debug, Serialize, FromRow, Deserialize)]
pub struct PluginConfig {
    pub id: i32,
    //协议名称
    pub name: String,
    //描述
    pub description: Option<String>,
    //自定义表单属性
    #[serde(rename = "formCustomization")]
    pub form_customization: Option<String>,
    // 插件类型
    #[serde(rename = "pluginType")]
    pub plugin_type: PluginType,
}

// 插件类型
#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum PluginType {
    // 系统插件
    #[serde(rename = "Protocol")]
    Protocol,
    // 自定义插件
    #[serde(rename = "DataOutput")]
    DataOutput,
    // 规则引擎
    #[serde(rename = "RuleEngine")]
    RuleEngine,
}

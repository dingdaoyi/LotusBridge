use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;
use protocol_core::{Point, PointWithProtocolId, Value};
use rand::{Rng, thread_rng};

#[derive(Debug, Clone)]
pub struct DeviceGroupValue {
    pub id: i32,
    pub name: String,
    pub device_id: i32,
    pub point_values: Vec<PointValue>,
}


#[derive(Debug, Clone)]
pub struct PointValue {
    pub id: i32,
    pub point: Point,
    pub value: Option<Value>,
}

impl From<Point> for PointValue {
    fn from(point: Point) -> Self {
        Self {
            id: point.id,
            point,
            value: None,
        }
    }
}

impl From<PointWithProtocolId> for PointValue {
    fn from(point: PointWithProtocolId) -> Self {
        Self {
            id: point.point_id,
            point: Point {
                id: point.point_id,
                device_id: point.device_id,
                group_id: point.group_id,
                address: point.address,
                data_type: point.data_type,
                access_mode: point.access_mode,
                multiplier: point.multiplier,
                precision: point.precision,
                description: point.description,
                part_number: point.part_number,
            },
            value: None,
        }
    }
}

/// 数据导出配置
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ExportConfig {
    pub id: i32,
    //推送名称
    pub name: String,
    // 推送配置
    pub configuration: Json<HashMap<String, String>>,
    // 描述
    pub description: String,

    #[serde(rename = "pluginId")]
    pub plugin_id: i32,
}

/// 数据导出配置
#[derive(Debug, Deserialize, Clone)]
pub struct CreateExportConfig {
    pub name: String,
    // 推送配置
    pub configuration: Json<HashMap<String, String>>,
    // 描述
    pub description: String,

    #[serde(rename = "pluginId")]
    pub plugin_id: i32,
}


#[derive(Clone, Debug,Serialize,Deserialize,PartialEq)]
pub struct MqttConfigProperties {
    /// 用户名
   pub username: Option<String>,

    ///密码
    pub password: Option<String>,

    /// 连接地址 tcp://ip:port
    pub url: String,

    /// 客户端id
    pub client_id: String,

    pub keep_alive: u16,

    pub timeout: u16,

    pub auto_connect: bool,
}

impl MqttConfigProperties {
    pub fn new(configuration: HashMap<String, String>) -> Self {
        let username: Option<String> = configuration.get("username").cloned();
        let password = configuration.get("password").cloned();
        let url = configuration.get("url").cloned().unwrap_or("tcp://mqtt.diweiyunlian.cn:2840".to_string());
        let client_id = configuration.get("client_id").cloned().unwrap_or(generate_client_id(8));
        let keep_alive: u16 = configuration
            .get("keep_alive")
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(30);

        let timeout: u16 = configuration
            .get("timeout")
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(30);
        let auto_connect: bool = configuration
            .get("auto_connect")
            .and_then(|s| s.parse::<bool>().ok())
            .unwrap_or(false);
        Self {
            username,
            password,
            url,
            client_id,
            keep_alive,
            timeout,
            auto_connect,
        }
    }
}

fn generate_client_id(length: usize) -> String {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    let mut rng = thread_rng();

    let client_id: String = (0..length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect();

    client_id
}

#[test]
fn test_deserialize_from_json() {
    let json_data = r#"{
        "username": "john_doe",
        "password": "secret_password",
        "url": "tcp://mqtt.diweiyunlian.cn:2840",
        "client_id": "random_client_id",
        "keep_alive": "60",
        "timeout": "45",
        "auto_connect": "true"
    }"#;

    let config:HashMap<String,String> =
        serde_json::from_str(json_data).unwrap();
   let mqtt_config= MqttConfigProperties::new(config);
    let expected_mqtt_config = MqttConfigProperties {
        username: Some("john_doe".to_string()),
        password: Some("secret_password".to_string()),
        url: "tcp://mqtt.diweiyunlian.cn:2840".to_string(),
        client_id: "random_client_id".to_string(),
        keep_alive: 60,
        timeout: 45,
        auto_connect: true,
    };

    assert_eq!(mqtt_config, expected_mqtt_config);
}
use std::sync::{Arc, Mutex};
use std::time::Duration;
use paho_mqtt::{Client, ConnectOptionsBuilder, CreateOptions, DisconnectOptions};
use serde::Serialize;
use export_core::DataExport;
use export_core::model::{DeviceGroupValue, ExportConfig, MqttConfigProperties, PointValue};
use protocol_core::Value;

pub struct XiaozhiyunDataExport {
    client: Arc<Mutex<Option<Client>>>,
    topic_prefix: Option<String>,
}

impl Default for XiaozhiyunDataExport {
    fn default() -> Self {
        Self{
            client: Arc::new(Mutex::new(None)),
            topic_prefix: None,
        }
    }
}

impl XiaozhiyunDataExport {
    fn get_mut_prop_topic(&self, device_code: String) -> String {
        format!("{}/multi_prop/{}", self.topic_prefix.clone().unwrap_or("iottopic".into()), device_code)
    }
}

impl DataExport for XiaozhiyunDataExport {
    fn initialize(&mut self, config: ExportConfig) -> Result<(), String> {
        let conf = config.configuration.0;
        let config = conf.clone();
        let conf: MqttConfigProperties = MqttConfigProperties::new(conf);
        let client = client_from_config(conf).map_err(|e| e.to_string())?;
        self.client.lock().unwrap().get_or_insert(client);
        self.topic_prefix = config.get("topic").cloned();
        Ok(())
    }

    fn stop(&self, force: bool) -> Result<(), String> {
        // if let Some(mut client) = self.client.lock().unwrap().take() {
        //     client.disconnect().await.map_err(|e| e.to_string())?;
        // }
        let mut ee = self.client.lock().unwrap();
        let options = DisconnectOptions::default();
        ee.as_mut().unwrap().disconnect(options).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn export(&self, device_group_value: DeviceGroupValue) -> Result<(), String> {
        let topic=self.get_mut_prop_topic(format!("{}",&device_group_value.id));
        let client = self.client.lock();
        let client = client.map_err(|o| o.to_string())?/*ok_or_else(Err("获取锁错误").into())?*/;
        let message: MultiMessage = device_group_value.into();
        let json = serde_json::to_vec(&message).map_err(|e| e.to_string())?;
       let msg= paho_mqtt::Message::new(topic,json,1);
        client.clone().unwrap().publish(msg).unwrap();
        Ok(())
    }
}

fn client_from_config(config: MqttConfigProperties) -> Result<Client, paho_mqtt::Error> {
    let create_opts = CreateOptions::new();
    let client = Client::new(create_opts)?;
    let uris = vec![config.url.as_str()];

    let opts = ConnectOptionsBuilder::new()
        .server_uris(&uris)
        .keep_alive_interval(Duration::from_secs(config.keep_alive as u64))
        .clean_session(true)
        .user_name(config.username.unwrap_or("client".to_string()))
        .password(config.password.unwrap_or("password".to_string()))
        .finalize();

    client.connect(opts)?;
    Ok(client)
}


#[derive(Debug,Serialize)]
struct Message{
    id: Option<u16>,
    identifier: String,
    value: Option<Value>,
    #[serde(rename = "baseMessage")]
    base_message:BaseMessage
}

impl Message {
    pub fn new(point_value: PointValue) ->Self{
        let point = point_value.point;
        Self{
            id: Some(1),
            identifier: "pressure".to_string(),
            value: point_value.value,
            base_message: BaseMessage {
                unit_address: point.part_number,
                unit_type: 192,
                system_address: 1,
                system_type: 1,
                unit_discretion: point.description,
            },
        }
    }
}
#[derive(Debug,Serialize)]
#[serde(rename_all = "camelCase")]
struct BaseMessage{
    unit_address: Option<String>,
    unit_type: u16,
    system_address: u16,
    system_type: u16,
    unit_discretion: String,
}
#[derive(Debug,Serialize)]
struct MultiMessage{
    id:u16,
    #[serde(rename = "multiData")]
    multi_data:Vec<Message>,
}

impl From<DeviceGroupValue> for MultiMessage{
    fn from(value: DeviceGroupValue) -> Self {
        let mut multi_data = vec![];
        for x in value.point_values.iter() {
            let message=Message::new(x.clone());
            multi_data.push(message);
        }
        Self{
            id: 1,
            multi_data,
        }
    }
}
#[test]
fn test_deserialize_from_json() {}
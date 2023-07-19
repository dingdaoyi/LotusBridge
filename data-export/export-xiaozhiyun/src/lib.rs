use std::sync::{Arc, Mutex};
use std::time::Duration;
use paho_mqtt::{Client, ConnectOptionsBuilder, CreateOptions, DisconnectOptions};
use export_core::DataExport;
use export_core::model::{DeviceGroupValue, ExportConfig, MqttConfigProperties};

struct XiaozhiyunDataExport{
    client: Arc<Mutex<Option<Client>>>,
}

impl DataExport for XiaozhiyunDataExport{
    fn initialize(&mut self, config: ExportConfig) -> Result<(), String> {
        let conf = config.configuration.0;
        let conf:MqttConfigProperties = MqttConfigProperties::new(conf);
        let client = client_from_config(conf).map_err(|e| e.to_string())?;
        self.client.lock().unwrap().get_or_insert(client);
        Ok(())
    }

    fn stop(&self, force: bool) -> Result<(), String> {
        // if let Some(mut client) = self.client.lock().unwrap().take() {
        //     client.disconnect().await.map_err(|e| e.to_string())?;
        // }
       let mut ee = self.client.lock().unwrap();
       let options=  DisconnectOptions::default();
        ee.as_mut().unwrap().disconnect(options).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn export(&self, device_group_value: DeviceGroupValue) -> Result<(), String> {
        todo!()
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



#[test]
fn test_deserialize_from_json() {

}
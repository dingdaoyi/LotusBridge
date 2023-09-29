use crate::config::auth::Auth;
use crate::config::database::DatabaseConfig;
use config::{Config, File, FileFormat};
use derive_getters::Getters;

mod database;
// 自定义日期解析
pub mod auth;
pub mod cache;
pub mod date_format;
pub mod db;
pub mod device_shadow;
pub mod error;

#[derive(Getters)]
pub struct EdgeConfig {
    data_base_config: DatabaseConfig,
    logger_level: String,
    auth: Auth,
    lib_path: String,
    server_port: u16,
    server_ip: String,
}

impl EdgeConfig {
    pub fn init_config() -> EdgeConfig {
        // 创建一个新的配置对象
        let settings = Config::builder()
            .add_source(File::with_name("conf/application").format(FileFormat::Yaml))
            .build()
            .unwrap();
        // 从配置中获取特定的值
        let database_config: DatabaseConfig = settings.get("database").unwrap();
        let logger_level: String = settings.get("logger.level").unwrap();
        let auth: Auth = settings.get("auth").unwrap();
        let lib_path: String = settings.get("plugin.libPath").unwrap();
        let server_port: u16 = settings.get("server.port").unwrap();
        let server_ip: String = settings.get("server.ip").unwrap_or("0.0.0.0".into());
        EdgeConfig {
            data_base_config: database_config,
            logger_level,
            auth,
            lib_path,
            server_port,
            server_ip,
        }
    }
}

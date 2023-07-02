use crate::config::database::DatabaseConfig;
use config::{Config, File, FileFormat};
use derive_getters::Getters;
mod database;
// 自定义日期解析
pub mod date_format;
pub mod error;
pub mod cache;

#[derive(Getters)]
pub struct EdgeConfig {
    data_base_config: DatabaseConfig,
    logger_level:String,
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
        EdgeConfig {
            data_base_config: database_config,
            logger_level
        }
    }
}

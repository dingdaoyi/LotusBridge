use std::env;
use yanbing_edge::config::auth::set_auth_config;
use yanbing_edge::config::EdgeConfig;
use yanbing_edge::run_app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = EdgeConfig::init_config();
    env::set_var("RUST_LOG", conf.logger_level());
    // 设置用户缓存
    set_auth_config(conf.auth().clone());
    tracing_subscriber::fmt::init();
    run_app(conf.data_base_config().sqlite_database_url()).await?;
    Ok(())
}

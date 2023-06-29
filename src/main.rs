use std::env;
use yanbing_edge::config::EdgeConfig;
use yanbing_edge::run_app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = EdgeConfig::init_config();
    env::set_var("RUST_LOG", conf.logger_level());
    tracing_subscriber::fmt::init();
    run_app(conf.data_base_config().sqlite_database_url()).await?;
    Ok(())
}

use std::env;
use yanbing_edge::config::auth::set_auth_config;
use yanbing_edge::config::EdgeConfig;
use yanbing_edge::run_app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    run_app().await?;
    Ok(())
}

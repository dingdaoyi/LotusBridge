use tracing::debug;
use lotus_bridge::run_app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_app().await?;
    debug!("启动服务成功!");
    Ok(())
}

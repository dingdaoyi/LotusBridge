use yanbing_edge::run_app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_app().await?;
    Ok(())
}

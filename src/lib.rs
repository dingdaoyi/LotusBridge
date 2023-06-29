pub mod config;
pub mod controller;
pub mod models;
pub mod service;
pub mod routes;

use tokio::net::TcpListener;
use tracing::info;
use sqlx::sqlite::{SqlitePool};

pub async fn run_app(database_url:String) -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePool::connect(&database_url)
        .await?;
    let app = routes::register(pool);

    // Run it with hyper
    let listener = TcpListener::bind("0.0.0.0:8000").await?;
    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app)
        .await?;
    Ok(())
}

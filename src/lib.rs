pub mod config;
pub mod handler;
pub mod models;
pub mod routers;
pub mod utils;
pub mod middleware;
pub mod initialize;

use std::env;
use axum::Router;
use sqlx::{Pool, Sqlite};
use tokio::net::TcpListener;
use tracing::info;
use sqlx::sqlite::{SqlitePool};
use crate::config::auth::set_auth_config;
use crate::config::EdgeConfig;
use crate::config::error::EdgeError;
use crate::initialize::protocol::init_protocol;

pub async fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let mut  conf = EdgeConfig::init_config();
    env::set_var("RUST_LOG", conf.logger_level());
    // 设置用户缓存
    set_auth_config(conf.auth().clone());
    tracing_subscriber::fmt::init();
    let database_url = conf.data_base_config().sqlite_database_url();
    let pool = SqlitePool::connect(&database_url)
        .await?;
    let app = match routers::register(pool.clone(),&conf) {
        Ok(router) => router,
        Err(EdgeError::Message(msg)) => {
            // tracing::error!();
            panic!("初始化路由失败{}", msg);
        }
        _ => panic!("初始化路由失败")
    };
    //初始化协议栈
    match init_protocol(pool.clone()).await {
        Err(EdgeError::Message(msg)) => {
            panic!("初始化协议失败{}", msg);
        }
        _ => {}
    }
    // Run it with hyper
    let listener = TcpListener::bind("0.0.0.0:8000").await?;
    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app)
        .await?;
    Ok(())
}

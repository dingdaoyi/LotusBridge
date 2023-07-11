pub mod config;
pub mod handler;
pub mod models;
pub mod routers;
pub mod utils;
pub mod middleware;
pub mod initialize;
use std::env;
use std::net::SocketAddr;
use sqlx::sqlite::{SqlitePool};
use crate::config::auth::set_auth_config;
use crate::config::EdgeConfig;
use crate::config::error::EdgeError;
use crate::initialize::protocol::init_protocol;

pub async fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let conf = EdgeConfig::init_config();
    env::set_var("RUST_LOG", conf.logger_level());
    // 设置用户缓存
    set_auth_config(conf.auth().clone());
    tracing_subscriber::fmt::init();
    let database_url = conf.data_base_config().sqlite_database_url();
    let pool = SqlitePool::connect(&database_url)
        .await?;
    let app = match routers::register(pool.clone()) {
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
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], conf.server_port().clone()));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

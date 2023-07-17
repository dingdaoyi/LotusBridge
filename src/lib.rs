pub mod config;
pub mod handler;
pub mod models;
pub mod routers;
pub mod utils;
pub mod middleware;
pub mod initialize;
use std::env;
use std::net::SocketAddr;
use crate::config::auth::set_auth_config;
use crate::config::{db, EdgeConfig};
use crate::config::error::EdgeError;
use crate::initialize::device_group::init_device_group;
use crate::initialize::protocol::init_protocol;

pub async fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let conf = EdgeConfig::init_config();
    env::set_var("RUST_LOG", conf.logger_level());
    // 设置用户缓存
    set_auth_config(conf.auth().clone());
    tracing_subscriber::fmt::init();
    let database_url = conf.data_base_config().sqlite_database_url();
    db::init_connections(database_url).await.expect("初始化数据库错误!");
   let pool= db::get_conn();
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
   match init_device_group().await {
        Err(EdgeError::Message(msg)) => {
            panic!("初始化定时拉取数据失败{}", msg);
        }
        _ => {}
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], conf.server_port().clone()));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

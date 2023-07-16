use once_cell::sync::OnceCell;
use sqlx::{Pool, Sqlite, SqlitePool};
use crate::config::error::{EdgeError, Result};

static POOLS: OnceCell<Pool<Sqlite>> = OnceCell::new();

/// 初始化连接
pub async fn init_connections(conn_string: String) -> Result<()> {
    println!("建立数据连接: {}", conn_string);
    let pool = SqlitePool::connect(&conn_string)
        .await.map_err(|_| EdgeError::Message("连接数据库错误".into()))?;
    POOLS.set(pool)
        .map_err(|_| EdgeError::Message("重复设置数据库连接".to_string()))
}

/// 获取连接
pub fn get_conn() -> Pool<Sqlite> {
    POOLS.get().unwrap().clone()
}

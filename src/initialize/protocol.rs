use sqlx::{Pool, Sqlite};
use crate::handler::plugin_handler::load_all_protocol;
use crate::config::error::Result;
use crate::config::cache::get_protocol_store;

//初始化协议
pub(crate) async fn init_protocol(pool: Pool<Sqlite>)->Result<()> {
   let protocols= load_all_protocol(pool.clone())
       .await?;
  let store=  get_protocol_store().unwrap();
    for protocol_config in protocols.iter() {
        //如果是系统插件,直接初始化
        store.load_protocol(protocol_config)?;
    }
    Ok(())
}
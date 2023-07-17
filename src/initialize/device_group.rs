use std::time::Duration;
use sqlx::{Pool, Sqlite};
use crate::config::db;
use crate::config::error::Result;
use crate::handler::device_handler::list_all_device_group;
use crate::handler::point_handler::read_point_group_value;

///初始化设备定时任务
pub(crate) async fn init_device_group() -> Result<()> {
    let group_list=list_all_device_group(db::get_conn()).await?;
    tokio::spawn(async move{
        for device_group in group_list.iter() {
            let mut interval = tokio::time::interval(Duration::from_secs(device_group.interval as u64));
            loop {
                // 下面的代码块将每隔几秒执行
                tokio::select! {
            _ = interval.tick() => {
               let res=  read_point_group_value(device_group.clone()).await ;
                        match res {
                           Ok(device_group)=>{},
                            Err(e)=>{  }
                        };
                    // println!("定时查询数据:{:#?}",res)
            }
        }
            }
        }
    });
     Ok(())
}
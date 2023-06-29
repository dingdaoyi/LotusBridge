use sqlx::SqlitePool;
use crate::config::error::EdgeError;
use crate::models::things::ProductFunc;


pub struct ProductFuncService {
    pool: SqlitePool,
}

impl ProductFuncService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_thing(&self, id: i64) -> Result<ProductFunc, EdgeError> {
        let product_func = sqlx::query_as::<_, ProductFunc>(
            " SELECT * FROM product_func  where id =$1 LIMIT 1"
        )
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(product_func)
    }
}
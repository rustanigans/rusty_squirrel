use anyhow::Result;
use mysql::{prelude::Queryable, PooledConn};

pub trait GetDatabase: Send + Sync
{
    fn get_connection(&self) -> Result<PooledConn>;
    fn query_drop(&self, sql: &str) -> Result<()>
    {
        Ok(self.get_connection()?.query_drop(sql)?)
    }
}

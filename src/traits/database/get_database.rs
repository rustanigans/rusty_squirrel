use anyhow::Result;
use mysql::PooledConn;

pub trait GetDatabase: Send + Sync
{
    fn get_connection(&self) -> Result<PooledConn>;
}

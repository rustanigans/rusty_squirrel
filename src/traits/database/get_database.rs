use crate::traits::{table::Table};
use anyhow::Result;
use mysql::PooledConn;

pub trait GetDatabase<T: Table>: Send + Sync
{
    fn get_connection(&self) -> Result<PooledConn>;
}

use crate::traits::*;
use anyhow::Result;
use mysql::PooledConn;

// Just a marker trait so we can coerce to a `dyn DatabaseInterface<T>` and use all the above trait fns
pub trait DatabaseInterface<T: Table>:
    QueryInterface<T> + DeleteInterface<T> + InsertInterface<T> + UpdateInterface<T> + Send + Sync
{
    fn get_connection(&self) -> Result<PooledConn>;
}

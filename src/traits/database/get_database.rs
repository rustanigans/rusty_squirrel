use crate::traits::{database::database_interface::DatabaseInterface, table::Table};
use anyhow::Result;
use mysql::PooledConn;

pub trait GetDatabase<T: Table>: Send + Sync
{
    fn get_db(&self) -> &dyn DatabaseInterface<T>;
    fn get_connection(&self) -> Result<PooledConn>
    {
        self.get_db().get_connection()
    }
}

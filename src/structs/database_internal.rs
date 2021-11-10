use super::*;
use crate::traits::GetDatabase;

#[derive(Clone)]
pub struct SquirrelDatabase
{
    connection_pool: Pool
}

impl SquirrelDatabase
{
    pub fn new(database_url: &str) -> Result<Self>
    {
        let options = mysql::Opts::from_url(database_url)?;
        Ok(Self { connection_pool: Pool::new_manual(1, 10, options)? })
    }
}

impl GetDatabase for SquirrelDatabase
{
    fn get_connection(&self) -> Result<PooledConn>
    {
        self.connection_pool.get_conn().map_err(|e| e.into())
    }
}

use super::*;

#[derive(Clone)]
pub struct DatabaseInternal
{
    connection_pool: Pool
}

impl DatabaseInternal
{
    pub fn new(database_url: &str) -> Result<Self>
    {
        let options = mysql::Opts::from_url(database_url)?;
        Ok(Self { connection_pool: Pool::new_manual(1, 10, options)? })
    }

    pub fn get_connection(&self) -> Result<PooledConn>
    {
        self.connection_pool.get_conn().map_err(|e| e.into())
    }
}

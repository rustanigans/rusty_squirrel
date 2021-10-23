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
        Ok(Self { connection_pool: Pool::new(options)? })
    }
}

impl<T: Table + Insertable + Updatable + Send + Sync> DatabaseInterface<T> for DatabaseInternal
{
    fn get_connection(&self) -> Result<PooledConn>
    {
        self.connection_pool
            .get_conn()
            .map_err(|e| anyhow!("Unable to get pooled connection: {}", e))
    }
}

impl<T: Table + Insertable + Send + Sync> InsertInterface<T> for DatabaseInternal
{
}

impl<T: Table + Updatable + Send + Sync> UpdateInterface<T> for DatabaseInternal
{
}

impl<T: Table + Send + Sync> QueryInterface<T> for DatabaseInternal
{
}

impl<T: Table + Send + Sync> DeleteInterface<T> for DatabaseInternal
{
}

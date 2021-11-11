use super::*;
use crate::traits::GetDatabase;
use mysql::{prelude::Queryable, Conn, Opts};
use std::sync::Arc;

#[derive(Clone)]
pub struct SquirrelDatabase
{
    pub db_name:     Arc<String>,
    connection_pool: Pool
}

pub struct SquirrelDatabaseConfig
{
    pub min_connections: usize,
    pub max_connections: usize
}
impl Default for SquirrelDatabaseConfig
{
    fn default() -> Self
    {
        Self { min_connections: 1,
               max_connections: 10 }
    }
}

impl SquirrelDatabase
{
    pub fn new(database_url: &str, config: Option<SquirrelDatabaseConfig>) -> Result<Self>
    {
        let options = mysql::Opts::from_url(database_url)?;
        let config = config.unwrap_or_default();
        Ok(Self { db_name:         Arc::new(options.get_db_name()
                                                   .ok_or_else(|| anyhow!("No database provided in url"))?
                                                   .to_string()),
                  connection_pool: Pool::new_manual(config.min_connections, config.max_connections, options)? })
    }

    pub fn new_with_db(database_url: &str, config: Option<SquirrelDatabaseConfig>) -> Result<Self>
    {
        let options1 = mysql::Opts::from_url(database_url)?;

        let config = config.unwrap_or_default();
        let database_name = options1.get_db_name()
                                    .ok_or_else(|| anyhow!("No database provided in url"))?
                                    .to_string();
        let options = mysql::OptsBuilder::new().user(options1.get_user())
                                               .pass(options1.get_pass())
                                               .ip_or_hostname(Some(options1.get_ip_or_hostname()))
                                               .tcp_port(options1.get_tcp_port());
        let mut conn = Conn::new(Opts::from(options))?;
        if !conn.select_db(&database_name)
        {
            conn.query_drop(format!("CREATE SCHEMA IF NOT EXISTS `{}` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci;", database_name))?;
        }
        Ok(Self { db_name:         Arc::new(database_name),
                  connection_pool: Pool::new_manual(config.min_connections, config.max_connections, options1)? })
    }

    pub fn drop_schema(self) -> Result<()>
    {
        Ok(self.connection_pool
               .get_conn()?
               .query_drop(format!("DROP SCHEMA IF EXISTS `{}`;", self.db_name))?)
    }
}

impl GetDatabase for SquirrelDatabase
{
    fn get_connection(&self) -> Result<PooledConn>
    {
        self.connection_pool.get_conn().map_err(|e| e.into())
    }
}

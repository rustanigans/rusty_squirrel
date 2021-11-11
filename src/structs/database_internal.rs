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

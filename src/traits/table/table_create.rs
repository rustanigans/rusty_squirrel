use crate::traits::{GetDatabase, Table};
use anyhow::Result;
use mysql::prelude::Queryable;

pub trait TableCreate: Table
{
    fn create_table_statement() -> String;
}

pub trait TableCreator: GetDatabase
{
    fn create_new_table<TABLE: TableCreate>(&self) -> Result<()>
    {
        self.get_connection()?
            .query_drop(TABLE::create_table_statement())
            .map_err(|e| anyhow::anyhow!("Unable To Create Table - '{}' - {}", TABLE::TABLE_NAME, e))
    }
}

impl<T: GetDatabase> TableCreator for T
{
}

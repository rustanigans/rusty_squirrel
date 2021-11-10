use crate::traits::{CollectionViewInterface, Table};
use anyhow::Result;
use mysql::prelude::Queryable;

pub trait TableCreate<T: Table>: CollectionViewInterface<T>
{
    fn create_table_statement<TABLE: Table>(&self) -> String;
    fn create_new_table<TABLE: Table>(&self) -> Result<()>
    {
        self.get_connection()?
            .query_drop(&self.create_table_statement::<TABLE>())
            .map_err(|e| anyhow::anyhow!("Unable To Create Table - '{}' - {}", TABLE::TABLE_NAME, e))
    }
}

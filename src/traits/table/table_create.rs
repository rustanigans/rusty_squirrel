use crate::traits::{CollectionViewInterface, Table};
use anyhow::Result;
use mysql::prelude::Queryable;

pub trait TableCreate<T: Table>: CollectionViewInterface<T>
{
    fn create_table_statement(&self) -> String;
    fn create_new_table(&self) -> Result<()>
    {
        self.get_connection()?
            .query_drop(&self.create_table_statement())
            .map_err(|e| anyhow::anyhow!("Unable To Create Table - '{}' - {}", T::TABLE_NAME, e))
    }
}

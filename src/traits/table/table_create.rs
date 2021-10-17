use crate::traits::{CollectionQueryInterface, Table};
use anyhow::Result;

pub trait TableCreate<T: Table>: CollectionQueryInterface<T> + Send + Sync
{
    fn create_table_statement(&self) -> String;
    fn create_new_table(&self) -> Result<()>
    {
        self.query_drop(&self.create_table_statement()).map_err(|e| {
                                                           anyhow::anyhow!("Unable To Create Table - '{}' - {}",
                                                                           T::TABLE_NAME,
                                                                           e)
                                                       })
    }
}

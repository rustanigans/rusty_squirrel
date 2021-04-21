use crate::traits::table::Table;
use crate::traits::query::collection_query_interface::CollectionQueryInterface;


pub trait TableCreate<T: Table>: CollectionQueryInterface<T>
{
    fn create_table_statement(&self) -> String;
    fn create_new_table(&mut self) -> anyhow::Result<()>
    {
        self.query_drop(&self.create_table_statement()).map_err(|e| anyhow::anyhow!("Unable To Create Table - '{}' - {}", T::TABLE_NAME, e))
    }
}
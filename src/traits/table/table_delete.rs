use crate::traits::table::Table;
use crate::traits::query::collection_query_interface::CollectionQueryInterface;

pub trait TableDelete<T: Table>: CollectionQueryInterface<T>
{
    fn delete_table(&mut self) -> anyhow::Result<()>
    {
        self.query_drop(&format!("DROP TABLE {}", T::TABLE_NAME)).map_err(|e| anyhow::anyhow!("Unable To Delete Table - '{}' - {}", T::TABLE_NAME, e))
    }
}
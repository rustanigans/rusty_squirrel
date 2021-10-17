use crate::traits::{CollectionQueryInterface, Table};
use anyhow::Result;

pub trait TableDelete<T: Table>: CollectionQueryInterface<T> + Send + Sync
{
    fn delete_table(&self) -> Result<()>
    {
        self.query_drop(&format!("DROP TABLE {}", T::TABLE_NAME))
            .map_err(|e| anyhow::anyhow!("Unable To Delete Table - '{}' - {}", T::TABLE_NAME, e))
    }
}

use crate::traits::{CollectionViewInterface, Table};
use anyhow::Result;
use mysql::prelude::Queryable;

pub trait TableDelete<T: Table>: CollectionViewInterface<T>
{
    fn delete_table(&self) -> Result<()>
    {
        self.get_connection()?
            .query_drop(&format!("DROP TABLE {}", T::TABLE_NAME))
            .map_err(|e| anyhow::anyhow!("Unable To Delete Table - '{}' - {}", T::TABLE_NAME, e))
    }
}

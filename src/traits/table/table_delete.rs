use crate::traits::{CollectionViewInterface, Table};
use anyhow::Result;
use mysql::prelude::Queryable;

pub trait TableDelete<T: Table>: CollectionViewInterface<T>
{
    fn delete_table<TABLE: Table>(&self) -> Result<()>
    {
        self.get_connection()?
            .query_drop(&format!("DROP TABLE {}", TABLE::TABLE_NAME))
            .map_err(|e| anyhow::anyhow!("Unable To Delete Table - '{}' - {}", TABLE::TABLE_NAME, e))
    }
}

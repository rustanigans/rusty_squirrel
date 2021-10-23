use crate::traits::{GetDatabase, Updatable};
use anyhow::Result;

pub trait CollectionUpdateInterface<T: Updatable>: GetDatabase<T> + Send + Sync
{
    fn update_column_by_id(&self, id: u64, changes: Vec<(String, String)>) -> Result<()>
    {
        self.get_db().update_column_by_id(id, changes)
    }

    fn update_item_by_id(&self, id: u64, item: &T) -> Result<()>
    {
        self.get_db().query_drop(&item.update_item_statement(id))
    }
}

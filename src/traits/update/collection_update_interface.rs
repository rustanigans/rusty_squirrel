use crate::traits::{GetDatabase, Table, Updatable};
use anyhow::Result;

pub trait CollectionUpdateInterface<T: Table + Updatable>: GetDatabase<T> + Send + Sync
{
    fn update_column_by_id(&self, id: u64, changes: Vec<(String, String)>) -> Result<()>
    {
        self.get_db().update_column_by_id(id, changes)
    }
}

use crate::traits::{GetDatabase, Table, Updatable};
use anyhow::Result;

pub trait CollectionUpdateInterface<T: Table + Updatable>: GetDatabase<T> + Send + Sync
{
    fn update_by_id(&self, id: u64, items: Vec<(String, String)>) -> Result<()>
    {
        self.get_db().update_by_id(id, items)
    }
}

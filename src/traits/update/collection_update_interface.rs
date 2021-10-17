use crate::traits::{GetDatabase, Table, Updatable};
use anyhow::Result;

pub trait CollectionUpdateInterface<T: Table + Updatable>: GetDatabase<T> + Send + Sync
{
    fn update_by_id(&self, id: u32, items: Vec<(String, String)>) -> Result<()>
    {
        self.get_db().lock().unwrap().update_by_id(id, items)
    }
}

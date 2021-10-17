use crate::traits::{GetDatabase, Table, Updatable};

pub trait CollectionUpdateInterface<T: Table + Updatable>: GetDatabase<T> + Send + Sync
{
    fn update_by_id(&self, id: u32, items: Vec<(String, String)>) -> anyhow::Result<()>
    {
        self.get_db().lock().unwrap().update_by_id(id, items)
    }
}

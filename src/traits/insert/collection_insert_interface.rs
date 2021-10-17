use crate::traits::{GetDatabase, Insertable, Table};
use anyhow::Result;

pub trait CollectionInsertInterface<T: Table + Insertable>: GetDatabase<T> + Send + Sync // + QueryInterface<T>
{
    fn insert(&self, item: &T, indexing_statement: Option<&str>) -> Result<u64>
    {
        self.get_db().lock().unwrap().insert(&item, indexing_statement)
    }

    fn insert_and_fetch(&self, item: T, indexing_statement: Option<&str>) -> Result<T>
    {
        self.get_db()
            .lock()
            .unwrap()
            .insert_and_fetch(&item, indexing_statement)
    }
}

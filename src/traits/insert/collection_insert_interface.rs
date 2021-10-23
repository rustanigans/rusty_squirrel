use crate::traits::{GetDatabase, Insertable, Table};
use anyhow::Result;

pub trait CollectionInsertInterface<T: Table + Insertable>: GetDatabase<T> + Send + Sync // + QueryInterface<T>
{
    fn insert_and_return_id(&self, item: &T) -> Result<u64>
    {
        self.get_db().insert_and_return_id(&item)
    }

    fn insert_and_return_id_with_indexing_check(&self, item: &T, indexing_statement: Option<&str>) -> Result<u64>
    {
        self.get_db()
            .insert_and_return_id_with_indexing_check(&item, indexing_statement)
    }

    fn insert_and_fetch(&self, item: T) -> Result<T>
    {
        self.get_db().insert_and_fetch(&item)
    }

    fn insert_and_fetch_with_indexing_check(&self, item: T, indexing_statement: Option<&str>) -> Result<T>
    {
        self.get_db()
            .insert_and_fetch_with_indexing_check(&item, indexing_statement)
    }

    fn update_item_by_id(&self, id: u64, item: &T) -> Result<u64>
    {
        self.get_db().update_item_by_id(id, item)
    }
}

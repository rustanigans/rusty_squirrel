use crate::traits::{GetDatabase, Insertable, Table};
use anyhow::Result;

pub trait CollectionInsertInterface<T: Table + Insertable>: GetDatabase<T> + Send + Sync // + QueryInterface<T>
{
    fn insert(&self, item: T, check_expression: Option<&str>) -> Result<u32>
    {
        self.get_db().lock().unwrap().insert(&item, check_expression)
    }

    fn insert_and_fetch(&self, item: T, check_expression: Option<&str>) -> Result<T>
    {
        self.get_db().lock().unwrap().insert_and_fetch(&item, check_expression)
    }
}

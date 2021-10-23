use crate::traits::insert::insertable::Insertable;
use anyhow::Result;

pub trait InsertInterface<T: Insertable>: Send + Sync
{
    fn insert_and_return_id(&self, item: &T) -> Result<u64>;

    fn insert_and_return_id_with_indexing_check(&self, item: &T, indexing_statement: Option<&str>) -> Result<u64>;

    fn insert_and_fetch(&self, item: &T) -> Result<T>;

    fn insert_and_fetch_with_indexing_check(&self, item: &T, indexing_statement: Option<&str>) -> Result<T>;

    fn update_item_by_id(&self, id: u64, item: &T) -> Result<u64>;
}

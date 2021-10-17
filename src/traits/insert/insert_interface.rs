use crate::traits::insert::insertable::Insertable;
use anyhow::Result;

pub trait InsertInterface<T: Insertable>: Send + Sync
{
    fn insert(&self, item: &T, indexing_statement: Option<&str>) -> Result<u64>;

    fn insert_and_fetch(&self, item: &T, indexing_statement: Option<&str>) -> Result<T>;
}

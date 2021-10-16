use crate::traits::insert::insertable::Insertable;

pub trait InsertInterface<T: Insertable>: Send + Sync
{
    fn insert(&self, item: &T, check_expression: Option<&str>) -> anyhow::Result<u32>;

    fn insert_and_fetch(&self, item: &T, check_expression: Option<&str>) -> anyhow::Result<T>;
}

use crate::traits::insert::insertable::Insertable;

pub trait InsertInterface<T: Insertable>
{
    fn insert(&mut self, item: &T, check_expression: Option<&str>) -> anyhow::Result<u32>;

    fn insert_and_fetch(&mut self, item: &T, check_expression: Option<&str>) -> anyhow::Result<T>;
}

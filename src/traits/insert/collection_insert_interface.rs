use crate::traits::table::Table;
use crate::traits::insert::insertable::Insertable;
use crate::traits::database::get_database::GetDatabase;


pub trait CollectionInsertInterface<T: Table + Insertable> : GetDatabase<T> // + QueryInterface<T>
{
    fn insert(&mut self, item: T, check_expression: Option<&str>) -> anyhow::Result<u32>
    {
        self.get_db().lock().unwrap().insert(&item, check_expression)
    }

    fn insert_and_fetch(&mut self, item: T, check_expression: Option<&str>) -> anyhow::Result<T>
    {
        self.get_db().lock().unwrap().insert_and_fetch(&item, check_expression)
    }
}
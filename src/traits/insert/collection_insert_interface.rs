use crate::traits::{database::get_database::GetDatabase, insert::insertable::Insertable,
                    table::Table};

pub trait CollectionInsertInterface<T: Table + Insertable>: GetDatabase<T> // + QueryInterface<T>
{
    fn insert(&self, item: T, check_expression: Option<&str>) -> anyhow::Result<u32>
    {
        self.get_db()
            .lock()
            .unwrap()
            .insert(&item, check_expression)
    }

    fn insert_and_fetch(&self, item: T, check_expression: Option<&str>) -> anyhow::Result<T>
    {
        self.get_db()
            .lock()
            .unwrap()
            .insert_and_fetch(&item, check_expression)
    }
}

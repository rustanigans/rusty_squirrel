use crate::traits::{database::get_database::GetDatabase, table::Table,
                    update::updatable::Updatable};

pub trait CollectionUpdateInterface<T: Table + Updatable>: GetDatabase<T>
{
    fn update_by_id(&self, id: u32, items: Vec<(String, String)>) -> anyhow::Result<()>
    {
        self.get_db().lock().unwrap().update_by_id(id, items)
    }
}

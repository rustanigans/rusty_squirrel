use crate::traits::table::Table;
use crate::traits::update::updatable::Updatable;
use crate::traits::database::get_database::GetDatabase;


pub trait CollectionUpdateInterface<T: Table + Updatable> : GetDatabase<T>
{
    fn update_by_id(&mut self, id: u32, items: Vec<(String, String)>) -> anyhow::Result<()>
    {
        self.get_db().lock().unwrap().update_by_id(id, items)
    }
}
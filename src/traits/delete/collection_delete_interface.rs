use crate::traits::table::Table;
use crate::traits::database::get_database::GetDatabase;


pub trait CollectionDeleteInterface<T: Table + Send + Sync> : GetDatabase<T>
{
    fn delete_by_id(&self, id: u32) -> anyhow::Result<()>
    {
        self.get_db().lock().unwrap().delete_by_id(id)
    }

    fn delete_by_expression(&self, expression: &str) -> anyhow::Result<u64>
    {
        self.get_db().lock().unwrap().delete_by_expression(expression)
    }
}
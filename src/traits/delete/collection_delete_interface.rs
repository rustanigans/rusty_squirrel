use crate::traits::{GetDatabase, Table};

pub trait CollectionDeleteInterface<T: Table>: GetDatabase<T> + Send + Sync
{
    fn delete_by_id(&self, id: u32) -> anyhow::Result<()>
    {
        self.get_db().lock().unwrap().delete_by_id(id)
    }

    fn delete_by_expression(&self, expression: &str) -> anyhow::Result<u64>
    {
        self.get_db()
            .lock()
            .unwrap()
            .delete_by_expression(expression)
    }
}

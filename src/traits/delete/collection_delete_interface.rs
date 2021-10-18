use crate::traits::{GetDatabase, Table};
use anyhow::Result;

pub trait CollectionDeleteInterface<T: Table>: GetDatabase<T> + Send + Sync
{
    fn delete_by_id(&self, id: u64) -> Result<()>
    {
        self.get_db().delete_by_id(id)
    }

    fn delete_by_expression(&self, expression: &str) -> Result<u64>
    {
        self.get_db().delete_by_expression(expression)
    }
}

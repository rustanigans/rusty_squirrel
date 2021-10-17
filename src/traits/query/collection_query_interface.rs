use crate::traits::{GetDatabase, Table};
use anyhow::Result;

pub trait CollectionQueryInterface<T: Table>: GetDatabase<T> + Send + Sync
{
    fn query_drop(&self, statement: &str) -> Result<()>
    {
        self.get_db().lock().unwrap().query_drop(statement)
    }

    fn query_all(&self) -> Result<Vec<T>>
    {
        self.get_db().lock().unwrap().query_all()
    }

    fn query_by_id(&self, id: u64) -> Result<T>
    {
        self.get_db().lock().unwrap().query_by_id(id)
    }

    fn query_by_expression(&self, expression: &str) -> Result<Vec<T>>
    {
        self.get_db().lock().unwrap().query_by_expression(expression)
    }
}

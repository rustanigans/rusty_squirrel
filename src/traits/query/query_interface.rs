use crate::traits::table::Table;
use anyhow::Result;

pub trait QueryInterface<T: Table>: Send + Sync
{
    fn query_drop(&self, statement: &str) -> Result<()>;

    fn query_all(&self) -> Result<Vec<T>>;

    fn query_by_id(&self, id: u32) -> Result<T>;

    fn query_by_expression(&self, expression: &str) -> Result<Vec<T>>;
}

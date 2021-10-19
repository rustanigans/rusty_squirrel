use crate::traits::table::Table;
use anyhow::Result;

pub trait DeleteInterface<T: Table>: Send + Sync
{
    fn delete_by_id(&self, id: u64) -> Result<()>;

    fn delete_by_expression(&self, expression: &str) -> Result<()>;
}

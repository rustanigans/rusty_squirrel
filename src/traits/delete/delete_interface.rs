use crate::traits::table::Table;

pub trait DeleteInterface<T: Table>: Send + Sync
{
    fn delete_by_id(&self, id: u32) -> anyhow::Result<()>;

    fn delete_by_expression(&self, expression: &str) -> anyhow::Result<u64>;
}

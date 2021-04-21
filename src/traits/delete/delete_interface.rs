use crate::traits::table::Table;



pub trait DeleteInterface<T: Table>
{
    fn delete_by_id(&mut self, id: u32) -> anyhow::Result<()>;

    fn delete_by_expression(&mut self, expression: &str) -> anyhow::Result<u64>;
}
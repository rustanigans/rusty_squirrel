use crate::traits::table::Table;

pub trait QueryInterface<T: Table>
{
    fn query_drop(&mut self, statement: &str) -> anyhow::Result<()>;
    
    fn query_all(&mut self) -> anyhow::Result<Vec<T>>;

    fn query_by_id(&mut self, id: u32) -> anyhow::Result<T>;

    fn query_by_expression(&mut self, expression: &str) -> anyhow::Result<Vec<T>>;
}
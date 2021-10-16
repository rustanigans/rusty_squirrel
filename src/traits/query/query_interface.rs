use crate::traits::table::Table;

pub trait QueryInterface<T: Table>: Send + Sync
{
    fn query_drop(&self, statement: &str) -> anyhow::Result<()>;

    fn query_all(&self) -> anyhow::Result<Vec<T>>;

    fn query_by_id(&self, id: u32) -> anyhow::Result<T>;

    fn query_by_expression(&self, expression: &str) -> anyhow::Result<Vec<T>>;
}

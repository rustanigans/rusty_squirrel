use crate::traits::table::Table;

pub trait QueryInterface<T: Table>: Send + Sync
{
}

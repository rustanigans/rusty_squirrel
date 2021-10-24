use crate::traits::Table;

pub trait InsertInterface<T: Table>: Send + Sync
{
}

use crate::traits::Table;

pub trait UpdateInterface<T: Table>: Send + Sync
{
}

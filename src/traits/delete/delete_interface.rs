use crate::traits::table::Table;

pub trait DeleteInterface<T: Table>: Send + Sync
{
}

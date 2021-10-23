use crate::traits::Insertable;

pub trait InsertInterface<T: Insertable>: Send + Sync
{
}

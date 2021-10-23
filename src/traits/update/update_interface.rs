use crate::traits::Updatable;

pub trait UpdateInterface<T: Updatable>: Send + Sync
{
}

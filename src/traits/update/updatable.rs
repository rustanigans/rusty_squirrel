use crate::traits::table::Table;

pub trait Updatable: Table + Send + Sync
{
}

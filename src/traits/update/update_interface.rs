use crate::traits::update::updatable::Updatable;
use anyhow::Result;

pub trait UpdateInterface<T: Updatable>: Send + Sync
{
    fn update_column_by_id(&self, id: u64, changes: Vec<(String, String)>) -> Result<()>;
}

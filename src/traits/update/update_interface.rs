use crate::traits::update::updatable::Updatable;
use anyhow::Result;

pub trait UpdateInterface<T: Updatable>: Send + Sync
{
    fn update_by_id(&self, id: u64, items: Vec<(String, String)>) -> Result<()>;
}

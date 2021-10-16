use crate::traits::update::updatable::Updatable;

pub trait UpdateInterface<T: Updatable + Send + Sync>
{
    fn update_by_id(&self, id: u32, items: Vec<(String, String)>) -> anyhow::Result<()>;
}

use crate::traits::update::updatable::Updatable;

pub trait UpdateInterface<T: Updatable>
{
    fn update_by_id(&mut self, id: u32, items: Vec<(String, String)>) -> anyhow::Result<()>;
}
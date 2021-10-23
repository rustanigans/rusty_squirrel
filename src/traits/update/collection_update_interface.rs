use crate::traits::{GetDatabase, Updatable};
use anyhow::{bail, Result};
use mysql::prelude::Queryable;

pub trait CollectionUpdateInterface<T: Updatable>: GetDatabase<T> + Send + Sync
{
    fn update_column_by_id(&self, id: u64, items: Vec<(String, String)>) -> Result<()>
    {
        let mut conn = self.get_connection()?;
        let id_statement = T::update_column_by_id_statement(id, items);

        let result = conn.query_drop(id_statement);

        match result
        {
            Ok(_) =>
            {
                let aff_rows = conn.affected_rows();
                if aff_rows == 1
                {
                    Ok(())
                }
                else
                {
                    bail!("Error - Failed To Update Item")
                }
            }
            Err(e) =>
            {
                bail!(e)
            }
        }
    }

    fn update_item_by_id(&self, id: u64, item: &T) -> Result<()>
    {
        self.get_connection()?
            .query_drop(&item.update_item_statement(id))
            .map_err(|e| e.into())
    }
}

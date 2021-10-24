use crate::traits::{GetDatabase, Updatable};
use anyhow::{bail, Result};
use mysql::{prelude::Queryable, PooledConn};

pub trait CollectionUpdateInterface<T: Updatable>: GetDatabase<T> + Send + Sync
{
    fn update_column_by_id(&self, id: u64, changes: Vec<(String, String)>) -> Result<()>
    {
        let mut conn = self.get_connection()?;
        let id_statement = T::update_column_by_id_statement(id, changes);

        let result = conn.query_drop(id_statement);

        check_update_result(result, &mut conn)
    }

    fn update_item_by_id(&self, id: u64, item: &T) -> Result<()>
    {
        let mut conn = self.get_connection()?;
        let id_statement = &item.update_item_by_id_statement(id);

        let query_by_id_statement = T::query_by_id_statement(id);
        println!("statement {:?}", query_by_id_statement);
        let result = conn.query_drop(query_by_id_statement);
        assert!(result.is_err());
        match result
        {
            Ok(_) =>
            {
                let result1 = conn.query_drop(id_statement);

                check_update_result(result1, &mut conn)
            }
            Err(e) =>
            {
                bail!(e)
            }
        }
    }
}

fn check_update_result(result: mysql::error::Result<()>, conn: &mut PooledConn) -> Result<()>
{
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

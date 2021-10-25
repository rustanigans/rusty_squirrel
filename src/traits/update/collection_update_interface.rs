use crate::traits::{GetDatabase, Updatable};
use anyhow::{bail, Result};
use mysql::{error as my_err, prelude::Queryable, PooledConn};

pub trait CollectionUpdateInterface<T: Updatable>: GetDatabase<T> + Send + Sync
{
    fn update_column_by_id(&self, id: u64, changes: Vec<(String, String)>) -> Result<()>
    {
        let mut conn = self.get_connection()?;
        let update_column_by_id_statement = T::update_column_by_id_statement(id, changes);
        let query_by_id_statement = T::query_by_id_statement(id);

        let result: my_err::Result<Option<T>> = conn.query_first(&query_by_id_statement);

        self.check_query_result(result, &update_column_by_id_statement, conn)
    }

    fn update_item_by_id(&self, id: u64, item: &T) -> Result<()>
    {
        let mut conn = self.get_connection()?;

        let update_item_by_id_statement = &item.update_item_by_id_statement(id);
        let query_by_id_statement = T::query_by_id_statement(id);

        let result: my_err::Result<Option<T>> = conn.query_first(&query_by_id_statement);

        self.check_query_result(result, update_item_by_id_statement, conn)
    }

    fn check_query_result(&self, result: my_err::Result<Option<T>>, stmt: &str, mut conn: PooledConn) -> Result<()>
    {
        match result
        {
            Ok(o) =>
            {
                match o
                {
                    None =>
                    {
                        bail!("Error - Cannot Update - Item Not Found")
                    }
                    Some(_) =>
                    {
                        let result1 = conn.query_drop(stmt);
                        check_update_result(result1, &mut conn)
                    }
                }
            }
            Err(e) =>
            {
                bail!(e)
            }
        }
    }
}

fn check_update_result(result: my_err::Result<()>, conn: &mut PooledConn) -> Result<()>
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

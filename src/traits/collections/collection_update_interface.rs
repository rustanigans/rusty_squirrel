use super::*;
use crate::traits::Updatable;

pub trait CollectionUpdateInterface: GetDatabase
{
    fn update_column_by_id<T: Updatable>(&self, id: u64, changes: Vec<(String, String)>) -> Result<()>
    {
        let mut conn = self.get_connection()?;
        let update_column_by_id_statement = T::update_column_by_id_statement(id, changes);
        let query_by_id_statement = T::query_by_id_statement(id);

        let result: mysql::error::Result<Option<T>> = conn.query_first(&query_by_id_statement);

        self.check_query_result(result, &update_column_by_id_statement, conn)
    }

    fn update_item_by_id<T: Updatable>(&self, id: u64, item: &T) -> Result<()>
    {
        let mut conn = self.get_connection()?;

        let update_item_by_id_statement = &item.generate_update_by_id_statement(id);
        let query_by_id_statement = T::query_by_id_statement(id);

        let result: mysql::error::Result<Option<T>> = conn.query_first(&query_by_id_statement);

        self.check_query_result(result, update_item_by_id_statement, conn)
    }

    fn check_query_result<T: Updatable>(&self,
                                        result: mysql::error::Result<Option<T>>,
                                        stmt: &str,
                                        mut conn: PooledConn)
                                        -> Result<()>
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

impl<DB: GetDatabase> CollectionUpdateInterface for DB
{
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

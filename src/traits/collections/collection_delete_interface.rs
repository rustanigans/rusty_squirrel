use super::*;

pub trait CollectionDeleteInterface: GetDatabase
{
    fn delete_by_id<T: Table>(&self, id: u64) -> Result<()>
    {
        let mut conn = self.get_connection()?;
        let id_statement = T::delete_by_id_statement(id);

        let result = conn.query_drop(id_statement);
        check_delete_result(result, &conn)
    }

    /// ```
    /// expression = &format!("`field_name` = '{}'", value);
    /// ```
    fn delete_by_expression<T: Table>(&self, expression: &str) -> Result<()>
    {
        let mut conn = self.get_connection()?;
        let expression_statement = T::delete_by_expression_statement(expression);

        let result = conn.query_drop(expression_statement);

        check_delete_result(result, &conn)
    }
}

impl<DB: GetDatabase> CollectionDeleteInterface for DB
{
}

fn check_delete_result(result: mysql::error::Result<()>, conn: &PooledConn) -> Result<()>
{
    match result
    {
        Ok(_) =>
        {
            if conn.affected_rows() == 1
            {
                Ok(())
            }
            else
            {
                bail!("Error - Failed To Delete Item")
            }
        }
        Err(e) =>
        {
            bail!(e)
        }
    }
}

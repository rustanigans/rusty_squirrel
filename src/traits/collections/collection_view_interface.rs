use super::*;
use mysql::prelude::Queryable;

pub trait CollectionViewInterface: GetDatabase
{
    /// ```
    /// expression = &format!("`field_name` = '{}'", value);
    /// ```
    fn query_by_expression<T: View>(&self, expression: &str) -> Result<Vec<T>>
    {
        let result = self.get_connection()?
                         .query(T::query_by_expression_statement(expression))?;
        Ok(result)
    }

    fn query_by_id_unchecked<T: View>(&self, id: u64) -> Result<Option<T>>
    {
        let mut conn = self.get_connection()?;
        let id_statement = T::query_by_id_statement(id);

        conn.query_first(&id_statement).map_err(|e| e.into())
    }

    fn query_all<T: View>(&self) -> Result<Vec<T>>
    {
        let mut conn = self.get_connection()?;
        conn.query(T::query_all_statement()).map_err(|e| e.into())
    }

    fn query_by_id<T: View>(&self, id: u64) -> Result<T>
    {
        let mut conn = self.get_connection()?;
        let id_statement = T::query_by_id_statement(id);

        let result: mysql::error::Result<Option<T>> = conn.query_first(&id_statement);
        match result
        {
            Ok(o) =>
            {
                match o
                {
                    None =>
                    {
                        bail!("Error - Query Failed - Item Not Found")
                    }
                    Some(p) => Ok(p)
                }
            }
            Err(e) =>
            {
                bail!(e)
            }
        }
    }

    fn query_first_by_id<T: View>(&self, id: u64) -> Result<Option<T>>
    {
        let mut conn = self.get_connection()?;
        let expression_statement = T::query_by_id_statement(id);
        conn.query_first(expression_statement).map_err(|e| e.into())
    }

    /// ```
    /// expression = &format!("`field_name` = '{}'", value);
    /// ```
    fn query_first_by_expression<T: View>(&self, expression: &str) -> Result<Option<T>>
    {
        let mut conn = self.get_connection()?;
        let expression_statement = T::query_by_expression_statement(expression);
        conn.query_first(expression_statement).map_err(|e| e.into())
    }
}

impl<DB: GetDatabase> CollectionViewInterface for DB
{
}

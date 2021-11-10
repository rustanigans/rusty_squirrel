use super::*;
use mysql::prelude::Queryable;

pub trait CollectionViewInterface<T: View>: GetDatabase
{
    fn query_by_expression(&self, expression: &str) -> Result<Vec<T>>
    {
        let result = self.get_connection()?
                         .query(T::query_by_expression_statement(expression))?;
        Ok(result)
    }

    fn query_by_id_unchecked(&self, id: u64) -> Result<Option<T>>
    {
        let mut conn = self.get_connection()?;
        let id_statement = T::query_by_id_statement(id);

        conn.query_first(&id_statement).map_err(|e| e.into())
    }

    fn query_all(&self) -> Result<Vec<T>>
    {
        let mut conn = self.get_connection()?;
        conn.query(T::query_all_statement()).map_err(|e| e.into())
    }

    fn query_by_id(&self, id: u64) -> Result<T>
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

    fn query_first_by_id(&self, id: u64) -> Result<Option<T>>
    {
        let mut conn = self.get_connection()?;
        let expression_statement = T::query_by_id_statement(id);
        conn.query_first(expression_statement).map_err(|e| e.into())
    }

    fn query_first_by_expression(&self, expression: &str) -> Result<Option<T>>
    {
        let mut conn = self.get_connection()?;
        let expression_statement = T::query_by_expression_statement(expression);
        conn.query_first(expression_statement).map_err(|e| e.into())
    }
}

impl<T: View, DB: GetDatabase> CollectionViewInterface<T> for DB
{
}

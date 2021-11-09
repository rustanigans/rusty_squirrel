use crate::traits::GetDatabase;
use anyhow::Result;
use mysql::prelude::{FromRow, Queryable};

pub trait View: FromRow + Send + Sync
{
    fn query_view_statement() -> String;
    fn query_options_statement() -> String
    {
        Default::default()
    }
    fn query_view_by_expression_statement(expression: &str) -> String
    {
        Self::query_view_statement() + " WHERE " + expression + " " + &Self::query_options_statement()
    }
    fn query_view_by_id_statement(id: u64) -> String
    {
        format!("{} WHERE id = {} {}",
                Self::query_view_statement(),
                id,
                Self::query_options_statement())
    }
}

pub trait CollectionViewInterface<T: View>: GetDatabase
{
    fn query_view_by_expression(&self, expression: &str) -> Result<Vec<T>>
    {
        let result = self.get_connection()?
                         .query(T::query_view_by_expression_statement(expression))?;
        Ok(result)
    }

    fn query_view_by_id_unchecked(&self, id: u64) -> Result<Option<T>>
    {
        let mut conn = self.get_connection()?;
        let id_statement = T::query_view_by_id_statement(id);

        conn.query_first(&id_statement).map_err(|e| e.into())
    }
}

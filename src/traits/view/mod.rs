use mysql::prelude::FromRow;

pub trait View: FromRow + Send + Sync
{
    const TABLE_NAME: &'static str;
    fn field_selection() -> String
    {
        "*".to_string()
    }

    fn join_clause() -> String
    {
        "".to_string()
    }

    fn from_clause() -> String
    {
        format!("`{}`", Self::TABLE_NAME)
    }

    fn query_statement() -> String
    {
        format!("SELECT {} FROM {} {}",
                Self::field_selection(),
                Self::from_clause(),
                Self::join_clause())
    }

    fn options_clause() -> String
    {
        Default::default()
    }

    fn where_clause(expression: &str) -> String
    {
        format!(" WHERE {} ", expression)
    }

    fn query_all_statement() -> String
    {
        Self::query_statement() + &Self::options_clause()
    }

    fn query_by_id_statement(id: u64) -> String
    {
        format!("{} {} {}",
                Self::query_statement(),
                Self::where_clause(&format!("id = {}", id)),
                Self::options_clause())
    }

    fn query_by_expression_statement(expression: &str) -> String
    {
        format!("{} {} {}",
                Self::query_statement(),
                Self::where_clause(expression),
                Self::options_clause())
    }
}

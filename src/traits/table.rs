pub mod table_create;
pub mod table_delete;

use mysql::prelude::FromRow;

pub trait Table: FromRow + Send + Sync
{
    const TABLE_NAME: &'static str;

    fn query_all_statement() -> String
    {
        format!("SELECT * FROM {}", Self::TABLE_NAME)
    }

    fn query_by_id_statement(id: u64) -> String
    {
        format!("SELECT * FROM {} WHERE id = {}", Self::TABLE_NAME, id)
    }

    fn query_by_expression_statement(expression: &str) -> String
    {
        format!("SELECT * FROM {} WHERE {}", Self::TABLE_NAME, expression)
    }

    fn insert_into_statement(expression: &str) -> String
    {
        format!("INSERT INTO {} {}", Self::TABLE_NAME, expression)
    }

    fn insert_into_on_duplicate_statement(expression: &str) -> String
    {
        format!("INSERT INTO {} {} ON DUPLICATE KEY UPDATE",
                Self::TABLE_NAME,
                expression)
    }

    fn delete_from_by_id_statement(id: u64) -> String
    {
        format!("DELETE FROM {} WHERE id = {}", Self::TABLE_NAME, id)
    }

    fn delete_by_expression_statement(expression: &str) -> String
    {
        format!("DELETE FROM {} WHERE {}", Self::TABLE_NAME, expression)
    }

    fn update_column_by_id_statement(id: u64, mut items: Vec<(String, String)>) -> String
    {
        let updates = items.drain(..)
                           .map(|x| format!("`{}` = '{}'", x.0, x.1))
                           .collect::<Vec<String>>()
                           .join(", ");

        format!("UPDATE {} SET {} WHERE id = {}", Self::TABLE_NAME, &updates, id)
    }
}

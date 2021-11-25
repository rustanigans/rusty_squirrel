mod updatable;

pub use updatable::*;

use crate::traits::View;

pub trait Table: View
{
    fn delete_by_id_statement(id: u64) -> String
    {
        format!("DELETE FROM `{}` WHERE id = {}", Self::TABLE_NAME, id)
    }

    fn delete_by_expression_statement(expression: &str) -> String
    {
        format!("DELETE FROM `{}` WHERE {}", Self::TABLE_NAME, expression)
    }
}

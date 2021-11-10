mod table_create;
mod table_delete;
mod updatable;

pub use table_create::*;
pub use table_delete::*;
pub use updatable::*;

use crate::traits::View;

pub trait Table: View
{
    fn delete_by_id_statement(id: u64) -> String
    {
        format!("DELETE FROM {} WHERE id = {}", Self::TABLE_NAME, id)
    }

    fn delete_by_expression_statement(expression: &str) -> String
    {
        format!("DELETE FROM {} WHERE {}", Self::TABLE_NAME, expression)
    }
}

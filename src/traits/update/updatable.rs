use crate::traits::Insertable;
use mysql::Params;

pub trait Updatable: Insertable + Send + Sync
{
    fn update_item_statement(&self, id: u64) -> String
    {
        let mut set_stmts = vec![];
        if let Params::Named(params) = self.to_params()
        {
            params.iter().for_each(|(field_name, field_value)| {
                             set_stmts.push(format!("`{}` = {}", field_name, field_value.as_sql(false)));
                         });
        }
        else
        {
            panic!("Params were not of named variety");
        }
        format!("UPDATE {} SET {} WHERE id = {};",
                Self::TABLE_NAME,
                set_stmts.join(","),
                id)
    }
}

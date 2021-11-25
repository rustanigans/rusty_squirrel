use super::*;
use mysql::Params;

pub trait Updatable: Table
{
    fn to_params(&self) -> mysql::Params;

    fn insert_into_statement(expression: &str) -> String
    {
        format!("INSERT INTO {} {}", Self::TABLE_NAME, expression)
    }

    fn update_column_by_id_statement(id: u64, mut items: Vec<(String, String)>) -> String
    {
        let updates = items.drain(..)
                           .map(|x| {
                               if x.1.to_uppercase() == "NULL"
                               {
                                   format!("`{}` = {}", x.0, x.1)
                               }
                               else
                               {
                                   format!("`{}` = '{}'", x.0, x.1)
                               }
                           })
                           .collect::<Vec<String>>()
                           .join(", ");

        format!("UPDATE {} SET {} WHERE id = {}", Self::TABLE_NAME, &updates, id)
    }

    fn update_by_id_statement(id: u64, expression: &str) -> String
    {
        format!("UPDATE {} SET {} WHERE id = {}", Self::TABLE_NAME, expression, id)
    }

    fn generate_update_by_id_statement(&self, id: u64) -> String
    {
        let mut set_expressions = vec![];
        if let Params::Named(params) = self.to_params()
        {
            params.iter().for_each(|(field_name, field_value)| {
                             set_expressions.push(format!("`{}` = {}", field_name, field_value.as_sql(false)));
                         });
        }
        else
        {
            panic!("Params were not of named variety");
        }
        Self::update_by_id_statement(id, &set_expressions.join(","))
    }

    fn generate_insert_expr(&self) -> String
    {
        let mut field_names_a = vec![];
        let mut field_names_b = vec![];
        if let Params::Named(params) = self.to_params()
        {
            for field_name in params.keys()
            {
                field_names_a.push(format!("`{}`", field_name));
                field_names_b.push(format!(":{}", field_name));
            }
        }
        else
        {
            panic!("Params were not of named variety");
        }
        format!("({}) VALUES ({})", field_names_a.join(", "), field_names_b.join(", "))
    }
}

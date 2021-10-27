use crate::traits::table::Table;
use mysql::Params;

pub trait Insertable: Table + Send + Sync
{
    // TODO move this to collection
    fn to_params(&self) -> mysql::Params;
    // TODO move this to collection
    fn insert_expr(&self) -> String
    {
        let mut field_names_a = vec![];
        let mut field_names_b = vec![];
        if let Params::Named(params) = self.to_params()
        {
            for field_name in params.keys()
            {
                field_names_a.push(field_name.to_string());
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

use crate::traits::table::Table;

pub trait Insertable: Table
{
    const INSERT_EXPRESSION: &'static str;

    fn to_params(&self) -> mysql::Params;
}
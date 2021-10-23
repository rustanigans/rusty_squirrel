use crate::traits::table::Table;

pub trait Insertable: Table + Send + Sync
{
    const INSERT_EXPRESSION: &'static str;

    const UPDATE_EXPRESSION: &'static str;

    fn to_params(&self) -> mysql::Params;
}

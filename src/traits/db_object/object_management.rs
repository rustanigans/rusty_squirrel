use super::*;
use crate::traits::GetDatabase;
use anyhow::Result;
use mysql::prelude::Queryable;

pub trait DbObjectManagement: GetDatabase
{
    fn create_object<OBJ: DbObject>(&self) -> Result<()>
    {
        self.get_connection()?
            .query_drop(OBJ::create_statement())
            .map_err(|e| anyhow::anyhow!("Unable To Create object - '{}' - {}", OBJ::create_statement(), e))
    }

    fn drop_object<OBJ: DbObject>(&self) -> Result<()>
    {
        self.get_connection()?.query_drop(OBJ::drop_statement()).map_err(|e| {
                                                                    anyhow::anyhow!("Unable To Drop object - '{}' - {}",
                                                                                    OBJ::drop_statement(),
                                                                                    e)
                                                                })
    }
}

impl<T: GetDatabase> DbObjectManagement for T
{
}

use crate::traits::{database::database_interface::DatabaseInterface, table::Table};

pub trait GetDatabase<T: Table>: Send + Sync
{
    fn get_db(&self) -> &dyn DatabaseInterface<T>;
}

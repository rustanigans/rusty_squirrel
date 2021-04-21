use crate::traits::database::database_interface::DatabaseInterface;
use crate::traits::table::Table;
use std::sync::{Mutex, Arc};

pub trait GetDatabase<T: Table>
{
    fn get_db(&self) -> Arc<Mutex<dyn DatabaseInterface<T>>>;
}
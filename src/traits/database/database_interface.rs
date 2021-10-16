use crate::traits::{delete::delete_interface::DeleteInterface,
                    insert::{insert_interface::InsertInterface, insertable::Insertable},
                    query::query_interface::QueryInterface,
                    update::{updatable::Updatable, update_interface::UpdateInterface}};

// Just a marker trait so we can coerce to a `dyn DatabaseInterface<T>` and use
// all the above trait fns
pub trait DatabaseInterface<T: Insertable + Updatable>:
    QueryInterface<T> + DeleteInterface<T> + InsertInterface<T> + UpdateInterface<T> + Send + Sync
{
}

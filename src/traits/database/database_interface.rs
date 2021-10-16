use crate::traits::delete::delete_interface::DeleteInterface;
use crate::traits::insert::insertable::Insertable;
use crate::traits::update::updatable::Updatable;
use crate::traits::query::query_interface::QueryInterface;
use crate::traits::insert::insert_interface::InsertInterface;
use crate::traits::update::update_interface::UpdateInterface;

// Just a marker trait so we can coerce to a `dyn DatabaseInterface<T>` and use all the above trait fns
pub trait DatabaseInterface<T: Insertable + Updatable>: QueryInterface<T> + DeleteInterface<T> + InsertInterface<T> + UpdateInterface<T>
{}
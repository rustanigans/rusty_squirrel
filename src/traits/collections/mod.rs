use crate::traits::{GetDatabase, Table, View};
use anyhow::Result;
use mysql::{prelude::Queryable, PooledConn};

mod collection_delete_interface;
mod collection_insert_interface;
mod collection_update_interface;
mod collection_view_interface;

pub use collection_delete_interface::*;
pub use collection_insert_interface::*;
pub use collection_update_interface::*;
pub use collection_view_interface::*;

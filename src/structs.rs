pub mod database_internal;
mod db_int_helper;

use crate::{structs::db_int_helper::{internal_delete_by_expression, internal_delete_by_id, internal_insert,
                                     internal_query_by_expression, internal_query_by_id, internal_update},
            traits::{Insertable, Updatable, *}};
use anyhow::{anyhow, bail, Result};
use mysql::{prelude::Queryable, Pool, PooledConn};

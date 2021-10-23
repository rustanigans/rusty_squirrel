pub mod database_internal;
mod db_int_helper;
/*structs::db_int_helper::{internal_delete_by_expression, internal_delete_by_id, internal_insert,
internal_query_by_expression, internal_query_by_id, internal_update_column,
internal_update_item},*/
use crate::traits::{Insertable, Updatable, *};
use anyhow::{anyhow, Result};
use mysql::{Pool, PooledConn};

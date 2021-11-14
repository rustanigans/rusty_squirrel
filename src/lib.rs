#![allow(clippy::suspicious_else_formatting)]

pub use structs::database_internal::{SquirrelDatabase, SquirrelDatabaseConfig};
// Backwards compatibility
pub use SquirrelDatabase as DatabaseInternal;

#[macro_use]
extern crate anyhow;

mod structs;
pub mod traits
{
    pub use collections::*;
    pub use get_database::*;
    pub use table::*;
    pub use taker::*;
    pub use view::*;

    mod collections;
    mod get_database;
    mod table;
    mod taker;
    mod view;
}
pub mod macros
{
    pub mod enum_u8;
}

pub const MYSQL_DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
pub use rs_proc_macros::{RustyEnum, RustyParams};

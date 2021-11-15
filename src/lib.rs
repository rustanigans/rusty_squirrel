#![allow(clippy::suspicious_else_formatting)]

#[cfg(feature = "default")]
pub use structs::database_internal::{SquirrelDatabase, SquirrelDatabaseConfig};

// Backwards compatibility
#[cfg(feature = "default")]
pub use SquirrelDatabase as DatabaseInternal;

#[cfg(feature = "default")]
#[macro_use]
extern crate anyhow;

#[cfg(feature = "default")]
mod structs;
#[cfg(feature = "default")]
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

#[cfg(feature = "default")]
pub mod macros
{
    pub mod enum_u8;
}
#[cfg(feature = "default")]
pub const MYSQL_DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[cfg(feature = "rs_proc_macros")]
pub use rs_proc_macros::{RustyEnum, RustyParams};

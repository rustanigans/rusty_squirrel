#![allow(clippy::suspicious_else_formatting)]
mod structs;
mod traits
{
    pub mod table;
    pub mod taker;
    pub mod database
    {
        pub mod database_interface;
        pub mod get_database;
    }
    pub mod query
    {
        pub mod collection_query_interface;
        pub mod query_interface;
    }
    pub mod insert
    {
        pub mod collection_insert_interface;
        pub mod insertable;
        pub mod insert_interface;
    }
    pub mod update
    {
        pub mod collection_update_interface;
        pub mod updatable;
        pub mod update_interface;
    }
    pub mod delete
    {
        pub mod collection_delete_interface;
        pub mod delete_interface;
    }
}

pub(crate) const MYSQL_DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub use structs::*;
pub use traits::*;
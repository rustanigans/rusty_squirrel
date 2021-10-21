#![allow(clippy::suspicious_else_formatting)]
mod structs;
pub mod traits
{
    mod table;
    mod taker;
    mod database
    {
        mod database_interface;
        mod get_database;

        pub use database_interface::*;
        pub use get_database::*;
    }
    mod query
    {
        mod collection_query_interface;
        mod query_interface;

        pub use collection_query_interface::*;
        pub use query_interface::*;
    }
    mod insert
    {
        mod collection_insert_interface;
        mod insert_interface;
        mod insertable;

        pub use collection_insert_interface::*;
        pub use insert_interface::*;
        pub use insertable::*;
    }
    mod update
    {
        mod collection_update_interface;
        mod updatable;
        mod update_interface;

        pub use collection_update_interface::*;
        pub use updatable::*;
        pub use update_interface::*;
    }
    mod delete
    {
        mod collection_delete_interface;
        mod delete_interface;

        pub use collection_delete_interface::*;
        pub use delete_interface::*;
    }

    pub use database::*;
    pub use delete::*;
    pub use insert::*;
    pub use query::*;
    pub use table::*;
    pub use taker::*;
    pub use update::*;
}
pub mod macros
{
    pub mod enum_u8;
}

pub const MYSQL_DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub use structs::database_internal::DatabaseInternal;

mod object_management;

pub use object_management::*;

pub trait DbObject: Send + Sync
{
    fn create_statement() -> String;
    fn drop_statement() -> String;
}

use rusty_squirrel::RustyParams;

#[derive(RustyParams, PartialEq, Debug)]
#[rs_view("simpleobject", table("./simpleobject.sql"))]
pub struct SimpleObject
{
    field1: String
}

impl SimpleObject
{
    pub fn new(field1: &str) -> Self
    {
        Self { field1: field1.to_string() }
    }

    pub fn field1(&self) -> &String
    {
        &self.field1
    }
}

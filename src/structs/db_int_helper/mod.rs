use super::*;
use mysql::prelude::FromRow;

pub fn internal_insert<T: Insertable>(item: &T, insert_statement: &str, conn: &mut PooledConn) -> Result<()>
{
    match conn.exec_drop(insert_statement, item.to_params())
    {
        Ok(o) => Ok(o),
        Err(e) =>
        {
            bail!(e)
        }
    }
}

pub fn internal_update<T: Updatable>(update_statement: &str, conn: &mut PooledConn) -> Result<()>
{
    match conn.query_drop(update_statement)
    {
        Ok(o) => Ok(o),
        Err(e) =>
        {
            bail!(e)
        }
    }
}

pub fn internal_query_by_id<T: FromRow>(id_statement: &str, conn: &mut PooledConn) -> Result<Option<T>>
{
    let result: Result<Option<T>, mysql::Error> = conn.query_first(id_statement);
    match result
    {
        Ok(o) => Ok(o),
        Err(e) =>
        {
            bail!(e)
        }
    }
}

pub fn internal_query_by_expression<T: FromRow>(expression_statement: &str, conn: &mut PooledConn) -> Result<Vec<T>>
{
    let result: Result<Vec<T>, mysql::Error> = conn.query(expression_statement);
    match result
    {
        Ok(o) => Ok(o),
        Err(e) =>
        {
            bail!(e)
        }
    }
}

pub fn internal_delete_by_id(id_statement: &str, conn: &mut PooledConn) -> Result<()>
{
    let result: Result<_, mysql::Error> = conn.query_drop(id_statement);
    match result
    {
        Ok(_) => Ok(()),
        Err(e) =>
        {
            bail!(e)
        }
    }
}

pub fn internal_delete_by_expression(expression_statement: &str, conn: &mut PooledConn) -> Result<()>
{
    let result: Result<_, mysql::Error> = conn.query_drop(expression_statement);
    match result
    {
        Ok(_) => Ok(()),
        Err(e) =>
        {
            bail!(e)
        }
    }
}

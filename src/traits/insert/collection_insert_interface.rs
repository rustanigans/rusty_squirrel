use crate::traits::{GetDatabase, Insertable, Table};
use anyhow::{bail, Result};
use mysql::{prelude::Queryable, PooledConn};

pub trait CollectionInsertInterface<T: Insertable>: GetDatabase + Send + Sync // + QueryInterface<T>
{
    fn insert_and_return_id(&self, item: &T) -> Result<u64>
    {
        let mut conn = self.get_connection()?;
        let insert_statement = T::insert_into_statement(&item.insert_expr());

        let result = conn.exec_drop(&insert_statement, item.to_params());
        check_insert_result_for_id(result, &conn)
    }

    fn insert_and_return_id_with_indexing_check(&self, item: &T, indexing_statement: Option<&str>) -> Result<u64>
    {
        let mut conn = self.get_connection()?;

        if let Some(s) = indexing_statement
        {
            let result: Vec<T> = conn.query(s)?;
            if !result.is_empty()
            {
                bail!("Failed To Insert - Entry Already Exists, Use Update Instead")
            }
            let insert_statement = T::insert_into_statement(&item.insert_expr());

            let result = conn.exec_drop(insert_statement, item.to_params());
            check_insert_result_for_id(result, &conn)
        }
        else
        {
            bail!("Cannot Insert Item With Indexing Check - Indexing Statement Is None")
        }
    }

    fn insert_and_fetch(&self, item: &T) -> Result<T>
    {
        let mut conn = self.get_connection()?;
        let insert_statement = T::insert_into_statement(&item.insert_expr());

        let result = conn.exec_drop(insert_statement, item.to_params());
        check_insert_result(result, &mut conn)
    }

    fn insert_and_fetch_with_indexing_check(&self, item: &T, indexing_statement: Option<&str>) -> Result<T>
    {
        let mut conn = self.get_connection()?;

        if let Some(s) = indexing_statement
        {
            let result: Vec<T> = conn.query(s)?;
            if !result.is_empty()
            {
                bail!("Failed To Insert - Entry Already Exists, Use Update Instead")
            }
            let insert_statement = T::insert_into_statement(&item.insert_expr());

            let result = conn.exec_drop(&insert_statement, item.to_params());
            check_insert_result(result, &mut conn)
        }
        else
        {
            bail!("Cannot Insert Item With Indexing Check - Indexing Statement Is None")
        }
    }
}

fn check_insert_result_for_id(result: mysql::error::Result<()>, conn: &PooledConn) -> Result<u64>
{
    match result
    {
        Ok(_) =>
        {
            if conn.affected_rows() == 1
            {
                Ok(conn.last_insert_id() as u64)
            }
            else
            {
                bail!("Error - Failed To Insert Item")
            }
        }
        Err(e) =>
        {
            bail!(e)
        }
    }
}

fn check_insert_result<T: Table>(result: mysql::error::Result<()>, conn: &mut PooledConn) -> Result<T>
{
    match result
    {
        Ok(_) =>
        {
            if conn.affected_rows() == 1
            {
                let id = conn.last_insert_id() as u64;
                let id_statement = T::query_by_id_statement(id);

                let result: mysql::error::Result<Option<T>> = conn.query_first(id_statement);
                result.map_err(|e| e.into()).map(|x| x.unwrap())
            }
            else
            {
                bail!("Error - Failed To Insert Item")
            }
        }
        Err(e) =>
        {
            bail!(e)
        }
    }
}

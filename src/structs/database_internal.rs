use crate::traits::*;
use anyhow::{anyhow, bail, Result};
use mysql::{prelude::Queryable, Pool, PooledConn};
use std::sync::{Mutex, MutexGuard};

pub struct DatabaseInternal
{
    connection: Mutex<PooledConn>
}

impl DatabaseInternal
{
    pub fn new(database_url: &str) -> Self
    {
        Self { connection: Mutex::new(Pool::new(database_url).unwrap().get_conn().unwrap()) }
    }

    fn get_connection(&self) -> MutexGuard<PooledConn>
    {
        self.connection.lock().unwrap()
    }
}

impl<T: Table + Insertable + Updatable + Send + Sync> DatabaseInterface<T> for DatabaseInternal
{
}

impl<T: Table + Insertable + Send + Sync> InsertInterface<T> for DatabaseInternal
{
    fn insert(&self, item: &T, check_expression: Option<&str>) -> Result<u32>
    {
        if check_expression.is_some()
        {
            let result: Vec<T> = self.query_by_expression(check_expression.unwrap())?;
            if !result.is_empty()
            {
                bail!("Failed To Insert - Entry Already Exists")
            }
        }

        self.get_connection()
            .exec_drop(T::insert_into_statement(T::INSERT_EXPRESSION), item.to_params())?;
        if self.get_connection().affected_rows() == 1
        {
            Ok(self.get_connection().last_insert_id() as u32)
        }
        else
        {
            bail!("Insert Failed")
        }
    }

    fn insert_and_fetch(&self, item: &T, check_expression: Option<&str>) -> Result<T>
    {
        let id = self.insert(item, check_expression)?;
        self.query_by_id(id)
    }
}

impl<T: Table + Updatable + Send + Sync> UpdateInterface<T> for DatabaseInternal
{
    // TODO: can we do this a better way
    fn update_by_id(&self, id: u32, items: Vec<(String, String)>) -> Result<()>
    {
        self.get_connection()
            .query_drop(T::update_by_id_statement(id, items))
            .map_err(|e| anyhow!("{}", e))?;
        if !self.get_connection().affected_rows() == 1
        {
            bail!("Could Not Update Item @ id - {}", id)
        }
        else
        {
            Ok(())
        }
    }
}

impl<T: Table + Send + Sync> QueryInterface<T> for DatabaseInternal
{
    fn query_drop(&self, statement: &str) -> Result<()>
    {
        self.get_connection()
            .query_drop(statement)
            .map_err(|e| anyhow!("{}", e))
    }

    fn query_all(&self) -> Result<Vec<T>>
    {
        self.get_connection()
            .query(T::query_all_statement())
            .map_err(|e| anyhow!("{}", e))
    }

    fn query_by_id(&self, id: u32) -> Result<T>
    {
        self.get_connection()
            .query_first(T::query_by_id_statement(id))?
            .ok_or_else(|| anyhow!("Entry {} NOT Found! 404", id))
    }

    fn query_by_expression(&self, expression: &str) -> Result<Vec<T>>
    {
        let statement = T::query_by_expression_statement(expression);
        self.get_connection().query(statement).map_err(|e| anyhow!("{}", e))
    }
}

impl<T: Table + Send + Sync> DeleteInterface<T> for DatabaseInternal
{
    fn delete_by_id(&self, id: u32) -> Result<()>
    {
        self.get_connection().query_drop(T::delete_from_by_id_statement(id))?;

        if self.get_connection().affected_rows() == 1
        {
            Ok(())
        }
        else
        {
            bail!("Could NOT Delete, Entry NOT Found @ {}", id);
        }
    }

    fn delete_by_expression(&self, expression: &str) -> Result<u64>
    {
        let statement = T::delete_by_expression_statement(expression);
        self.get_connection()
            .query_drop(statement)
            .map_err(|e| anyhow!("{}", e))?;
        Ok(self.get_connection().affected_rows())
    }
}

use mysql::prelude::Queryable;
use mysql::{PooledConn, Pool};
use anyhow::*;
use crate::traits::table::Table;
use crate::traits::insert::insertable::Insertable;
use crate::traits::update::updatable::Updatable;
use crate::traits::database::database_interface::DatabaseInterface;
use crate::traits::insert::insert_interface::InsertInterface;
use crate::traits::update::update_interface::UpdateInterface;
use crate::traits::query::query_interface::QueryInterface;
use crate::traits::delete::delete_interface::DeleteInterface;


const URL: &str = "mysql://tz:sN%5EFtc%5EmpMN27J@trade-bot-db1.cbdfs5u4tcer.ap-northeast-1.rds.amazonaws.com/";

pub struct DatabaseInternal
{
    connection: PooledConn
}

impl DatabaseInternal
{
    pub fn new(database_name: &str) -> Self
    {
        let url = URL.to_string() + database_name;

        Self
        {
            connection: Pool::new(url).unwrap().get_conn().unwrap()
        }
    }
}

impl<T: Table + Insertable + Updatable> DatabaseInterface<T> for DatabaseInternal
{}

impl<T: Table + Insertable> InsertInterface<T> for DatabaseInternal
{
    fn insert(&mut self, item: &T, check_expression: Option<&str>)-> anyhow::Result<u32>
    {
        if check_expression.is_some()
        {
            let result: Vec<T> = self.query_by_expression(check_expression.unwrap())?;
            if !result.is_empty()
            {
                bail!("Failed To Insert - Entry Already Exists")
            }
        }

        self.connection.exec_drop(T::insert_into_statement(T::INSERT_STMT), item.to_params())?;
        if self.connection.affected_rows() == 1
        {
            Ok(self.connection.last_insert_id() as u32)
        }
        else
        {
            bail!("Insert Failed")
        }
    }

    fn insert_and_fetch(&mut self, item: &T, check_expression: Option<&str>) -> Result<T>
    {
        let id = self.insert(item, check_expression)?;
        self.query_by_id(id)
    }
}

impl<T: Table + Updatable> UpdateInterface<T> for DatabaseInternal
{
    // TODO: can we do this a better way
    fn update_by_id(&mut self, id: u32, items: Vec<(String, String)>) -> anyhow::Result<()>
    {
        self.connection.query_drop(T::update_by_id_statement(id, items)).map_err(|e| anyhow!("{}", e))?;
        if !self.connection.affected_rows() == 1
        {
            bail!("Could Not Update Item @ id - {}", id)
        }
        else { Ok(()) }
    }
}

impl<T: Table> QueryInterface<T> for DatabaseInternal
{
    fn query_drop(&mut self, statement: &str) -> anyhow::Result<()>
    {
        self.connection
            .query_drop(statement)
            .map_err(|e| anyhow!("{}", e))
    }

    fn query_all(&mut self) -> anyhow::Result<Vec<T>>
    {
        self.connection.query(T::query_all_statement()).map_err(|e| anyhow!("{}", e))
    }

    fn query_by_id(&mut self, id: u32) -> anyhow::Result<T>
    {
        self.connection.query_first(T::query_by_id_statement(id))?.ok_or_else(||anyhow!("Entry {} NOT Found! 404", id))
    }

    fn query_by_expression(&mut self, expression: &str) -> anyhow::Result<Vec<T>>
    {
        let statement = T::query_by_expression_statement(expression);
        self.connection.query(statement).map_err(|e| anyhow!("{}", e))
    }
}

impl<T: Table> DeleteInterface<T> for DatabaseInternal
{
    fn delete_by_id(&mut self, id: u32) -> anyhow::Result<()>
    {
        self.connection.query_drop(T::delete_from_by_id_statement(id))?;

        if self.connection.affected_rows() == 1
        {
            Ok(())
        }
        else
        {
            bail!("Could NOT Delete, Entry NOT Found @ {}", id);
        }
    }

    fn delete_by_expression(&mut self, expression: &str) -> anyhow::Result<u64>
    {
        let statement = T::delete_by_expression_statement(expression);
        self.connection.query_drop(statement).map_err(|e| anyhow!("{}", e))?;
        Ok(self.connection.affected_rows())
    }
}
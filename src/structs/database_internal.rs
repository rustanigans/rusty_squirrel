use super::*;

#[derive(Clone)]
pub struct DatabaseInternal
{
    connection_pool: Pool
}

impl DatabaseInternal
{
    pub fn new(database_url: &str) -> Result<Self>
    {
        let options = mysql::Opts::from_url(database_url)?;
        Ok(Self { connection_pool: Pool::new(options)? })
    }

    fn get_connection(&self) -> Result<PooledConn>
    {
        self.connection_pool
            .get_conn()
            .map_err(|e| anyhow!("Unable to get pooled connection: {}", e))
    }
}

impl<T: Table + Insertable + Updatable + Send + Sync> DatabaseInterface<T> for DatabaseInternal
{
}

impl<T: Table + Insertable + Send + Sync> InsertInterface<T> for DatabaseInternal
{
    fn insert_and_return_id(&self, item: &T) -> Result<u64>
    {
        let mut conn = self.get_connection()?;
        let insert_statement = T::insert_into_statement(T::INSERT_EXPRESSION);

        check_insert_result_for_id::<T>(internal_insert(item, &insert_statement, &mut conn), &conn)
    }

    fn insert_and_return_id_with_indexing_check(&self, item: &T, indexing_statement: Option<&str>) -> Result<u64>
    {
        let mut conn = self.get_connection()?;

        if indexing_statement.is_some()
        {
            let result: Vec<T> = internal_query_by_expression(indexing_statement.unwrap(), &mut conn)?;
            if !result.is_empty()
            {
                bail!("Failed To Insert - Entry Already Exists, Use Update Instead")
            }

            let insert_statement = T::insert_into_statement(T::INSERT_EXPRESSION);
            check_insert_result_for_id::<T>(internal_insert(item, &insert_statement, &mut conn), &conn)
        }
        else
        {
            bail!("Cannot Insert Item With Indexing Check - Indexing Statement Is None")
        }
    }

    fn insert_and_fetch(&self, item: &T) -> Result<T>
    {
        let mut conn = self.get_connection()?;
        let insert_statement = T::insert_into_statement(T::INSERT_EXPRESSION);

        check_insert_result(internal_insert(item, &insert_statement, &mut conn), &mut conn)
    }

    fn insert_and_fetch_with_indexing_check(&self, item: &T, indexing_statement: Option<&str>) -> Result<T>
    {
        let mut conn = self.get_connection()?;

        if indexing_statement.is_some()
        {
            let result: Vec<T> = internal_query_by_expression(indexing_statement.unwrap(), &mut conn)?;
            if !result.is_empty()
            {
                bail!("Failed To Insert - Entry Already Exists, Use Update Instead")
            }

            let insert_statement = T::insert_into_statement(T::INSERT_EXPRESSION);

            check_insert_result(internal_insert(item, &insert_statement, &mut conn), &mut conn)
        }
        else
        {
            bail!("Cannot Insert Item With Indexing Check - Indexing Statement Is None")
        }
    }

    fn update_item_by_id(&self, id: u64, item: &T) -> Result<u64>
    {
        let mut conn = self.get_connection()?;
        let id_statement = T::query_by_id_statement(id);
        println!("id statement {:?}", id_statement);

        println!("rusty - item id = {}", id);

        let result: Result<Option<T>> = internal_query_by_id(&id_statement, &mut conn);
        match result
        {
            Ok(o) =>
            {
                match o
                {
                    None =>
                    {
                        bail!("Error - Cannot Update - Item Not Found")
                    }
                    Some(p) =>
                    {
                        let update_by_id_statement = p.replace_statement(id);
                        println!("insert statement {:?}", update_by_id_statement);

                        check_insert_result_for_id::<T>(internal_update_item(item, &update_by_id_statement, &mut conn),
                                                        &conn)
                    }
                }
            }
            Err(e) =>
            {
                bail!(e)
            }
        }
    }
}

impl<T: Table + Updatable + Send + Sync> UpdateInterface<T> for DatabaseInternal
{
    fn update_column_by_id(&self, id: u64, items: Vec<(String, String)>) -> Result<()>
    {
        let mut conn = self.get_connection()?;
        let id_statement = T::update_column_by_id_statement(id, items);

        match internal_update_column::<T>(&id_statement, &mut conn)
        {
            Ok(_) =>
            {
                let aff_rows = conn.affected_rows();
                if aff_rows == 1
                {
                    Ok(())
                }
                else
                {
                    bail!("Error - Failed To Update Item")
                }
            }
            Err(e) =>
            {
                bail!(e)
            }
        }
    }
}

impl<T: Table + Send + Sync> QueryInterface<T> for DatabaseInternal
{
    fn query_drop(&self, statement: &str) -> Result<()>
    {
        let mut conn = self.get_connection()?;
        conn.query_drop(statement).map_err(|e| anyhow!("{}", e))
    }

    fn query_all(&self) -> Result<Vec<T>>
    {
        let mut conn = self.get_connection()?;
        conn.query(T::query_all_statement()).map_err(|e| anyhow!("{}", e))
    }

    fn query_by_id(&self, id: u64) -> Result<T>
    {
        let mut conn = self.get_connection()?;
        let id_statement = T::query_by_id_statement(id);
        let result: Result<Option<T>> = internal_query_by_id(&id_statement, &mut conn);
        match result
        {
            Ok(o) =>
            {
                match o
                {
                    None =>
                    {
                        bail!("Error - Query Failed - Item Not Found")
                    }
                    Some(p) => Ok(p)
                }
            }
            Err(e) =>
            {
                bail!(e)
            }
        }
    }

    fn query_by_expression(&self, expression: &str) -> Result<Vec<T>>
    {
        let mut conn = self.get_connection()?;
        let expression_statement = T::query_by_expression_statement(expression);
        internal_query_by_expression(&expression_statement, &mut conn)
    }
}

impl<T: Table + Send + Sync> DeleteInterface<T> for DatabaseInternal
{
    fn delete_by_id(&self, id: u64) -> Result<()>
    {
        let mut conn = self.get_connection()?;
        let id_statement = T::delete_from_by_id_statement(id);

        check_delete_result(internal_delete_by_id(&id_statement, &mut conn), &conn)
    }

    fn delete_by_expression(&self, expression: &str) -> Result<()>
    {
        let mut conn = self.get_connection()?;
        let expression_statement = T::delete_by_expression_statement(expression);
        check_delete_result(internal_delete_by_expression(&expression_statement, &mut conn), &conn)
    }
}

fn check_insert_result_for_id<T: Insertable>(result: Result<()>, conn: &PooledConn) -> Result<u64>
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

fn check_insert_result<T: Insertable>(result: Result<()>, conn: &mut PooledConn) -> Result<T>
{
    match result
    {
        Ok(_) =>
        {
            if conn.affected_rows() == 1
            {
                let id = conn.last_insert_id() as u64;
                let id_statement = T::query_by_id_statement(id);
                let item: Result<Option<T>> = internal_query_by_id(&id_statement, conn);
                match item
                {
                    Ok(o) => Ok(o.unwrap()),
                    Err(e) =>
                    {
                        bail!(e)
                    }
                }
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

fn check_delete_result(result: Result<()>, conn: &PooledConn) -> Result<()>
{
    match result
    {
        Ok(_) =>
        {
            if conn.affected_rows() == 1
            {
                Ok(())
            }
            else
            {
                bail!("Error - Failed To Delete Item")
            }
        }
        Err(e) =>
        {
            bail!(e)
        }
    }
}

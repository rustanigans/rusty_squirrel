mod helpers;

#[cfg(test)]
mod tests
{
    use crate::helpers::SimpleObject;
    use rusty_squirrel::{traits::{CollectionDeleteInterface, CollectionInsertInterface, CollectionUpdateInterface,
                                  CollectionViewInterface, DbObjectManagement},
                         SquirrelDatabase};

    #[test]
    fn creating_and_dropping_database_should_not_fail()
    {
        let db = SquirrelDatabase::new_with_db(&format!("{}{}", std::env::var("Test_DB").expect("env var Test_DB"), 0),
                                               None).unwrap();
        db.drop_schema().expect("Drop Schema");
    }

    #[test]
    fn creating_and_dropping_table_should_not_fail()
    {
        let db = SquirrelDatabase::new_with_db(&format!("{}{}", std::env::var("Test_DB").expect("env var Test_DB"), 1),
                                               None).expect("Failed to create database");
        db.create_object::<SimpleObject>().expect("Failed to create object");
        db.drop_object::<SimpleObject>().expect("Failed to delete object");

        db.drop_schema().expect("Drop Schema");
    }

    #[test]
    fn insert_and_return_id_should_not_fail()
    {
        let db = SquirrelDatabase::new_with_db(&format!("{}{}", std::env::var("Test_DB").expect("env var Test_DB"), 2),
                                               None).unwrap();
        db.create_object::<SimpleObject>().expect("Failed to create object");

        let object = SimpleObject::new("field1_value");
        let _ = db.insert_and_return_id(&object)
                  .expect("Failed to insert object and return id");

        db.drop_schema().expect("Drop Schema");
    }

    #[test]
    fn insert_and_fetch_should_not_fail()
    {
        let db = SquirrelDatabase::new_with_db(&format!("{}{}", std::env::var("Test_DB").expect("env var Test_DB"), 3),
                                               None).unwrap();
        db.create_object::<SimpleObject>().expect("Failed to create object");

        let object = SimpleObject::new("twiddle_sticks");
        let fetched_object = db.insert_and_fetch(&object).expect("Failed to insert and fetch object");
        assert_eq!(object, fetched_object);

        db.drop_schema().expect("Drop Schema");
    }

    #[test]
    fn delete_by_id_should_not_fail()
    {
        let db = SquirrelDatabase::new_with_db(&format!("{}{}", std::env::var("Test_DB").expect("env var Test_DB"), 4),
                                               None).unwrap();
        db.create_object::<SimpleObject>().expect("Failed to create object");

        let object = SimpleObject::new("twiddle_sticks");
        let id = db.insert_and_return_id(&object).expect("Failed to delete object by id");

        db.delete_by_id::<SimpleObject>(id)
          .expect("Failed to delete object by id");

        db.drop_schema().expect("Drop Schema");
    }

    #[test]
    fn delete_by_expression_should_not_fail()
    {
        let db = SquirrelDatabase::new_with_db(&format!("{}{}", std::env::var("Test_DB").expect("env var Test_DB"), 5),
                                               None).unwrap();
        db.create_object::<SimpleObject>().expect("Failed to create object");

        let object = SimpleObject::new("twiddle_sticks");
        let _ = db.insert_and_return_id(&object)
                  .expect("Failed to insert object and return id");

        db.delete_by_expression::<SimpleObject>(&format!("`field1` = '{}'", object.field1()))
          .expect("Failed to delete object by expression");

        db.drop_schema().expect("Drop Schema");
    }

    #[test]
    fn update_column_by_id_should_not_fail()
    {
        let db = SquirrelDatabase::new_with_db(&format!("{}{}", std::env::var("Test_DB").expect("env var Test_DB"), 6),
                                               None).unwrap();
        db.create_object::<SimpleObject>().expect("Failed to create object");

        let object = SimpleObject::new("twiddle_sticks");
        let id = db.insert_and_return_id(&object)
                   .expect("Failed to insert object and return id");

        let result = db.update_column_by_id::<SimpleObject>(id, vec![("field1".to_string(),
                                                                      "sticks_of_twiddle".to_string())])
                       .expect("Failed to update column by id");
        assert!(result);
        db.drop_schema().expect("Drop Schema");
    }

    #[test]
    fn update_item_by_id_should_not_fail()
    {
        let db = SquirrelDatabase::new_with_db(&format!("{}{}", std::env::var("Test_DB").expect("env var Test_DB"), 7),
                                               None).unwrap();
        db.create_object::<SimpleObject>().expect("Failed to create object");

        let object = SimpleObject::new("twiddle_sticks");
        let id = db.insert_and_return_id(&object)
                   .expect("Failed to insert object and return id");

        let result = db.update_item_by_id::<SimpleObject>(id, &SimpleObject::new("sticks_of_twiddle"))
                       .expect("Failed to update object by id");
        assert!(result);
        db.drop_schema().expect("Drop Schema");
    }

    #[test]
    fn query_by_id_should_not_fail()
    {
        let db = SquirrelDatabase::new_with_db(&format!("{}{}", std::env::var("Test_DB").expect("env var Test_DB"), 8),
                                               None).unwrap();
        db.create_object::<SimpleObject>().expect("Failed to create object");

        let object = SimpleObject::new("twiddle_sticks");
        let id = db.insert_and_return_id(&object)
                   .expect("Failed to insert object and return id");

        let returned_object = db.query_by_id::<SimpleObject>(id)
                                .expect("Failed to query object by id");
        assert_eq!(returned_object, object);
        db.drop_schema().expect("Drop Schema");
    }

    #[test]
    fn query_by_id_unchecked_should_not_fail()
    {
        let db = SquirrelDatabase::new_with_db(&format!("{}{}", std::env::var("Test_DB").expect("env var Test_DB"), 9),
                                               None).unwrap();
        db.create_object::<SimpleObject>().expect("Failed to create object");

        let object = SimpleObject::new("twiddle_sticks");
        let id = db.insert_and_return_id(&object)
                   .expect("Failed to insert object and return id");

        let returned_object = db.query_by_id_unchecked::<SimpleObject>(id)
                                .expect("Failed to query object by id_unchecked");
        assert_eq!(returned_object.unwrap(), object);
        db.drop_schema().expect("Drop Schema");
    }

    #[test]
    fn query_by_expression_should_not_fail()
    {
        let db =
            SquirrelDatabase::new_with_db(&format!("{}{}", std::env::var("Test_DB").expect("env var Test_DB"), 10),
                                          None).unwrap();
        db.create_object::<SimpleObject>().expect("Failed to create object");

        let object = SimpleObject::new("twiggie_sticks");
        let _ = db.insert_and_return_id(&object)
                  .expect("Failed to insert object and return id");

        let returned_object = db.query_by_expression::<SimpleObject>(&format!("`field1` = '{}'", object.field1()))
                                .expect("Failed to query object by expression");
        assert_eq!(returned_object[0], object);
        db.drop_schema().expect("Drop Schema");
    }

    #[test]
    fn query_all_should_not_fail()
    {
        let db =
            SquirrelDatabase::new_with_db(&format!("{}{}", std::env::var("Test_DB").expect("env var Test_DB"), 11),
                                          None).unwrap();
        db.create_object::<SimpleObject>().expect("Failed to create object");

        for i in 0..3
        {
            let object = SimpleObject::new(&format!("stick {}", i));
            let _ = db.insert_and_return_id(&object)
                      .expect("Failed to insert object and return id");
        }

        let returned_object = db.query_all::<SimpleObject>().expect("Failed to query all");
        assert_eq!(returned_object.len(), 3);
        assert_eq!(returned_object[1].field1(), &"stick 1".to_string());
        db.drop_schema().expect("Drop Schema");
    }

    #[test]
    fn query_first_by_id_should_not_fail()
    {
        let db =
            SquirrelDatabase::new_with_db(&format!("{}{}", std::env::var("Test_DB").expect("env var Test_DB"), 12),
                                          None).unwrap();
        db.create_object::<SimpleObject>().expect("Failed to create object");

        for i in 0..3
        {
            let object = SimpleObject::new(&format!("stick {}", i));
            let _ = db.insert_and_return_id(&object)
                      .expect("Failed to insert object and return id");
        }

        let returned_object = db.query_first_by_id::<SimpleObject>(1)
                                .expect("Failed to query first by id");
        assert_eq!(returned_object.unwrap().field1(), &"stick 0".to_string());
        db.drop_schema().expect("Drop Schema");
    }

    #[test]
    fn query_first_by_expression_should_not_fail()
    {
        let db =
            SquirrelDatabase::new_with_db(&format!("{}{}", std::env::var("Test_DB").expect("env var Test_DB"), 13),
                                          None).unwrap();
        db.create_object::<SimpleObject>().expect("Failed to create object");

        for i in 0..3
        {
            let object = SimpleObject::new(&format!("stick {}", i));
            let _ = db.insert_and_return_id(&object)
                      .expect("Failed to insert object and return id");
        }

        let returned_object = db.query_first_by_expression::<SimpleObject>(&format!("`field1` = '{}'", "stick 2"))
                                .expect("Failed to query first by expression");
        assert_eq!(returned_object.unwrap().field1(), &"stick 2".to_string());
        db.drop_schema().expect("Drop Schema");
    }

    #[test]
    fn insert_and_fetch_with_indexing_check_should_not_fail()
    {
        todo!()
    }

    #[test]
    fn insert_and_return_id_with_indexing_check_should_not_fail()
    {
        todo!()
    }
}

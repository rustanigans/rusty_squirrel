use std::sync::{Arc, Mutex};
use crate::structs::candles::Candles;
use crate::structs::database_internal::DatabaseInternal;
use crate::structs::orders::Orders;

pub struct DatabaseRoot
{
    pub db: Arc<Mutex<DatabaseInternal>>,
    pub candles: Candles,
    pub orders: Orders,
}

impl DatabaseRoot
{
    pub fn new(database_name: &str) -> Self
    {
        let db = Arc::new(Mutex::new(DatabaseInternal::new(database_name)));
        let cd_weak_ref = Arc::downgrade(&db);
        let ord_weak_ref = Arc::downgrade(&db);

        Self
        {
            db,
            candles: Candles::new(cd_weak_ref),
            orders: Orders::new(ord_weak_ref),
        }
    }
}


use crate::MYSQL_DATE_FORMAT;
use mysql::{chrono::{DateTime, TimeZone, Utc},
            prelude::FromValue,
            FromRowError, Row};
use std::convert::{TryFrom, TryInto};

pub trait Taker: Send + Sync
{
    fn take_hinted<T: FromValue>(&mut self, name: &str) -> Result<T, FromRowError>;
    fn take_enum<T: TryFrom<u8>>(&mut self, name: &str) -> Result<T, FromRowError>;
    fn take_date_time(&mut self, name: &str) -> Result<DateTime<Utc>, FromRowError>;
    fn take_date_time_option(&mut self, name: &str) -> Result<Option<DateTime<Utc>>, FromRowError>;
}
impl Taker for Row
{
    fn take_hinted<T: FromValue>(&mut self, name: &str) -> Result<T, FromRowError>
    {
        self.take::<T, &str>(name).ok_or_else(|| FromRowError(self.clone()))
    }

    fn take_enum<T: TryFrom<u8>>(&mut self, name: &str) -> Result<T, FromRowError>
    {
        self.take::<u8, &str>(name)
            .ok_or_else(|| FromRowError(self.clone()))?
            .try_into()
            .map_err(|_| FromRowError(self.clone()))
    }

    fn take_date_time(&mut self, name: &str) -> Result<DateTime<Utc>, FromRowError>
    {
        Utc.datetime_from_str(self.take::<String, &str>(name)
                                  .ok_or_else(|| FromRowError(self.clone()))?
                                  .as_str(),
                              MYSQL_DATE_FORMAT)
           .map_err(|_| FromRowError(self.clone()))
    }

    fn take_date_time_option(&mut self, name: &str) -> Result<Option<DateTime<Utc>>, FromRowError>
    {
        Ok(self.take::<Option<String>, &str>(name)
               .ok_or_else(|| FromRowError(self.clone()))?
               .map(|x| Utc.datetime_from_str(x.as_str(), MYSQL_DATE_FORMAT).unwrap()))
    }
}

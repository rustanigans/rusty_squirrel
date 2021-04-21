use mysql::prelude::FromValue;
use std::convert::{TryFrom, TryInto};
use mysql::{FromRowError, Row};
use mysql::chrono::{DateTime, Utc, TimeZone};
use crate::MYSQL_DATE_FORMAT;

pub trait Taker
{
    fn take_hinted<T: FromValue>(&mut self, name: &str) -> Result<T, FromRowError>;
    fn take_enum<T: TryFrom<usize>>(&mut self, name: &str) -> Result<T, FromRowError>;
    fn take_date_time(&mut self, name: &str) -> Result<DateTime<Utc>, FromRowError>;
    fn take_date_time_option(&mut self, name: &str) -> Result<Option<DateTime<Utc>>, FromRowError>;
}
impl Taker for Row
{
    fn take_hinted<T: FromValue>(&mut self, name: &str) -> Result<T, FromRowError>
    {
        self.take::<T, &str>(name)
            .ok_or_else(|| FromRowError(self.clone()))
    }

    fn take_enum<T: TryFrom<usize>>(&mut self, name: &str) -> Result<T, FromRowError>
    {
        self.take::<usize, &str>(name)
            .ok_or_else(|| FromRowError(self.clone()))?
            .try_into()
            .map_err(|_| FromRowError(self.clone()))
    }

    fn take_date_time(&mut self, name: &str) -> Result<DateTime<Utc>, FromRowError>
    {
        Utc.datetime_from_str(
            self.take::<String, &str>(name)
                .ok_or_else(|| FromRowError(self.clone()))?
                .as_str(),
            MYSQL_DATE_FORMAT
        )
           .map_err(|_| FromRowError(self.clone()))
    }

    fn take_date_time_option(&mut self, name: &str) -> Result<Option<DateTime<Utc>>, FromRowError>
    {
        Ok(self
            .take::<Option<String>, &str>(name)
            .ok_or_else(|| FromRowError(self.clone()))?
            .map(|x| {
                Utc.datetime_from_str(x.as_str(), MYSQL_DATE_FORMAT)
                   .unwrap()
            }))
    }
}
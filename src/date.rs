use std::result;
use std::str::FromStr;

use chrono::{
    DateTime, Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc,
};
use db::bson::Bson;
use serde::{de, Deserialize, Serialize, Serializer};

use errors::{Error, ErrorKind};

#[derive(Debug, Clone, Copy)]
pub struct Date(NaiveDate);

impl Date {
    pub fn value(&self) -> NaiveDate {
        self.0.clone()
    }

    pub fn today() -> Self {
        let today = Local::today();
        Self(NaiveDate::from_ymd(
            today.year(),
            today.month(),
            today.day(),
        ))
    }

    pub fn to_dt(&self) -> DateTime<Utc> {
        let datetime = NaiveDateTime::new(self.value(), NaiveTime::from_hms(0, 0, 0));
        Utc.from_local_datetime(&datetime).unwrap()
    }

    pub fn add_days(&mut self, days: i64) {
        self.0 = self.0 + Duration::days(days);
    }

    pub fn to_fmt_string(&self, fmt: &'static str) -> String {
        self.value().format(fmt).to_string()
    }
}

impl FromStr for Date {
    type Err = Error;
    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let date = NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map_err(|err| Error::new(err.to_string(), ErrorKind::InvalidData))?;
        Ok(Self(date))
    }
}

impl ToString for Date {
    fn to_string(&self) -> String {
        self.value().format("%Y-%m-%d").to_string()
    }
}

impl From<Date> for Bson {
    fn from(date: Date) -> Self {
        Bson::DateTime(date.to_dt().into())
    }
}

impl Into<DateTime<Utc>> for Date {
    fn into(self) -> DateTime<Utc> {
        let date = self.value();
        let time = NaiveTime::from_hms(0, 0, 0);
        let datetime = NaiveDateTime::new(date, time);
        Utc.from_local_datetime(&datetime).unwrap()
    }
}

impl From<db::bson::DateTime> for Date {
    fn from(item: db::bson::DateTime) -> Self {
        Self(NaiveDate::from_ymd(
            item.to_chrono().year(),
            item.to_chrono().month(),
            item.to_chrono().day(),
        ))
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("Date", &self.to_string())
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

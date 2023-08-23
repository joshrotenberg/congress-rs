use crate::sort::{date_serialize, Sort};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Parameters {
    pub limit: Option<u32>,
    offset: Option<u32>,
    #[serde(
        rename(serialize = "fromDateTime"),
        serialize_with = "date_serialize",
        skip_serializing_if = "Option::is_none"
    )]
    pub from_date: Option<DateTime<Utc>>,
    #[serde(
        rename(serialize = "toDateTime"),
        serialize_with = "date_serialize",
        skip_serializing_if = "Option::is_none"
    )]
    pub to_date: Option<DateTime<Utc>>,
    pub sort: Option<Sort>,
}

impl FromStr for Parameters {
    type Err = serde_urlencoded::de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_urlencoded::from_str::<Parameters>(s)
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

pub trait HasParameters: private::Sealed + Sized {
    fn get_parameters(&mut self) -> &mut Parameters;
}

pub trait PageParameters: HasParameters + Sized {
    fn limit(mut self, limit: u32) -> Self {
        self.get_parameters().limit = Some(limit);
        self
    }

    fn offset(mut self, offset: u32) -> Self {
        self.get_parameters().offset = Some(offset);
        self
    }
}

pub trait SortParameters: HasParameters + Sized {
    fn from_date(mut self, from_date: DateTime<Utc>) -> Self {
        self.get_parameters().from_date = Some(from_date);
        self
    }

    fn to_date(mut self, to_date: DateTime<Utc>) -> Self {
        self.get_parameters().to_date = Some(to_date);
        self
    }

    fn sort(mut self, sort: Sort) -> Self {
        self.get_parameters().sort = Some(sort);
        self
    }
}

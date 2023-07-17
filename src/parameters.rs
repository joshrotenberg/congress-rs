use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, Serializer};

const DATE_FORMAT: &str = "%Y-%m-%dT%H:%M:%SZ";

pub(crate) fn date_serialize<S>(
    date: &Option<DateTime<Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.unwrap().format(DATE_FORMAT));
    serializer.serialize_str(&s)
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Sort {
    /// Sort by the item's update date in ascending order
    // #[serde(rename(serialize = "updateDate+asc"))]
    UpdateDateAscending,
    /// Sort by the item's update date in ascending order
    // #[serde(rename(serialize = "updateDate+desc"))]
    UpdateDateDescending,
}

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UpdateDateAscending => write!(f, "updateDate+asc"),
            Self::UpdateDateDescending => write!(f, "updateDate+desc"),
        }
    }
}

#[derive(Debug, Deserialize, Default, Serialize)]
pub(crate) struct Parameters {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
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

pub(crate) mod macros {

    macro_rules! implement_page_params {
    ($($name:ident),+) => {
        $(
        use chrono::{DateTime, Utc};
        use crate::parameters::Sort;
        impl<'client> $name<'client> {
            pub fn limit(mut self, limit: u32) -> Self {
                self.params.limit = Some(limit);
                self
            }

            pub fn offset(mut self, offset: u32) -> Self {
                self.params.offset = Some(offset);
                self
            }

            pub fn from_date(mut self, date: DateTime<Utc>) -> Self {
                self.params.from_date = Some(date);
                self
            }

            pub fn to_date(mut self, date: DateTime<Utc>) -> Self {
                self.params.to_date = Some(date);
                self
            }

            pub fn sort(mut self, sort: Sort) -> Self
        {
            self.params.sort = Some(sort);
            self
        }
        }
        )+};
}
    pub(crate) use implement_page_params;
}

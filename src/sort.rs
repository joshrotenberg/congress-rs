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

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum Sort {
    /// Sort by the item's update date in ascending order
    #[serde(rename(serialize = "updateDate asc"))]
    UpdateDateAscending,
    /// Sort by the item's update date in ascending order
    #[serde(rename(serialize = "updateDate desc"))]
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

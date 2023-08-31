use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestAction {
    pub action_date: NaiveDate,
    pub action_time: Option<NaiveTime>,
    pub text: String,
}

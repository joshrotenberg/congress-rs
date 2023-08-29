use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestAction {
    pub action_date: NaiveDate,
    pub text: String,
}

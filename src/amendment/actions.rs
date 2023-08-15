use super::AmendmentHandler;
use crate::{chamber::Chamber, page::Pagination, Result};
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordedVote {
    pub chamber: Chamber,
    pub congress: u32,
    pub date: DateTime<Utc>,
    pub roll_number: u32,
    pub session_number: u32,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceSystem {
    pub code: u32,
    pub name: String,
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub enum ActionType {
    Floor,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub action_date: NaiveDate,
    pub recorded_votes: Option<Vec<RecordedVote>>,
    pub source_system: Option<SourceSystem>,
    pub text: Option<String>,
    pub action_type: Option<ActionType>,
}

#[derive(Debug, Deserialize)]
pub struct ActionsResponse {
    pub actions: Vec<Action>,
    pub pagination: Pagination,
}

#[derive(Debug)]
pub struct ActionsHandler<'client> {
    handler: &'client AmendmentHandler<'client>,
}

impl<'client> ActionsHandler<'client> {
    pub(super) fn new(handler: &'client AmendmentHandler) -> Self {
        Self { handler }
    }

    pub async fn send(&self) -> Result<ActionsResponse> {
        let path = format!(
            "/v3/amendment/{}/{}/{}/actions",
            self.handler.congress, self.handler.amendment_type, self.handler.amendment_number
        );

        self.handler
            .client
            .get(&path, Some(&self.handler.params))
            .await
    }
}

use super::AmendmentHandler;
use crate::{amendment_type::AmendmentType, page::Pagination, Result};
use chrono::NaiveDate;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct LatestAction {
    pub date: NaiveDate,
    pub text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amendment {
    pub congress: u32,
    pub latest_action: Option<LatestAction>,
    pub number: String,
    pub purpose: Option<String>,
    #[serde(rename = "type")]
    pub amendment_type: AmendmentType,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
pub struct AmendmentsResponse {
    pub amendments: Vec<Amendment>,
    pub pagination: Pagination,
}

#[derive(Debug)]
pub struct AmendmentsHandler<'client> {
    handler: &'client AmendmentHandler<'client>,
}

impl<'client> AmendmentsHandler<'client> {
    pub(super) fn new(handler: &'client AmendmentHandler) -> Self {
        Self { handler }
    }

    pub async fn send(&self) -> Result<AmendmentsResponse> {
        let path = format!(
            "/v3/amendment/{}/{}/{}/amendments",
            self.handler.congress, self.handler.amendment_type, self.handler.amendment_number
        );

        self.handler
            .client
            .get(&path, Some(&self.handler.params))
            .await
    }
}

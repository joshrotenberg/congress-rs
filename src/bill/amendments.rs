use super::BillHandler;
use crate::{amendment_type::AmendmentType, page::Pagination, Result};
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestAction {
    pub action_date: NaiveDate,
    pub action_time: NaiveTime,
    pub text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amendment {
    pub congress: u32,
    pub description: String,
    pub latest_action: LatestAction,
    pub number: String,
    #[serde(rename = "type")]
    pub amendment_type: AmendmentType,
    pub update_date: DateTime<Utc>,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
pub struct AmendmentsResponse {
    pub amendments: Vec<Amendment>,
    pub pagination: Pagination,
}

#[derive(Debug)]
pub struct AmendmentsHandler<'client> {
    handler: &'client BillHandler<'client>,
}

impl<'client> AmendmentsHandler<'client> {
    pub(super) fn new(handler: &'client BillHandler) -> Self {
        Self { handler }
    }

    pub async fn send(&self) -> Result<AmendmentsResponse> {
        let path = format!(
            "/v3/bill/{}/{}/{}/amendments",
            self.handler.congress, self.handler.bill_type, self.handler.bill_number
        );

        self.handler
            .client
            .get(&path, Some(&self.handler.params))
            .await
    }
}

crate::parameters::macros::implement_page_params!(AmendmentsHandler);
crate::page::macros::implement_paged_response!(AmendmentsResponse, Amendment, amendments);

use super::AmendmentHandler;
use crate::Result;
use chrono::NaiveDate;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cosponsor {
    pub bioguide_id: String,
    pub first_name: String,
    pub full_name: String,
    pub is_original_cosponsor: bool,
    pub last_name: String,
    pub party: String,
    pub sponsorship_date: NaiveDate,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
pub struct CosponsorsResponse {
    pub cosponsors: Vec<Cosponsor>,
}

#[derive(Debug)]
pub struct CosponsorsHandler<'client> {
    handler: &'client AmendmentHandler<'client>,
}

impl<'client> CosponsorsHandler<'client> {
    pub(super) fn new(handler: &'client AmendmentHandler) -> Self {
        Self { handler }
    }

    pub async fn send(&self) -> Result<CosponsorsResponse> {
        let path = format!(
            "/v3/amendment/{}/{}/{}/cosponsors",
            self.handler.congress, self.handler.amendment_type, self.handler.amendment_number
        );

        self.handler
            .client
            .get(&path, Some(&self.handler.params))
            .await
    }
}

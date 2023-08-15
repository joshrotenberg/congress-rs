use crate::{
    amendment_type::{self, AmendmentType},
    page::Pagination,
    parameters::Parameters,
    Client, Result,
};
use chrono::NaiveDate;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amendment {
    pub congress: u32,
    pub latest_action: LatestAction,
    pub number: String,
    pub purpose: Option<String>,
    #[serde(rename = "type")]
    pub amendment_type: AmendmentType,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestAction {
    pub action_date: NaiveDate,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct AmendmentsResponse {
    pub amendments: Vec<Amendment>,
    pub pagination: Pagination,
}

#[derive(Debug)]
pub struct AmendmentsHandler<'client> {
    client: &'client Client,
    congress: Option<u32>,
    amendment_type: Option<AmendmentType>,
    params: Parameters,
}

impl<'client> AmendmentsHandler<'client> {
    pub fn new(client: &'client Client) -> Self {
        AmendmentsHandler {
            client,
            congress: None,
            amendment_type: None,
            params: Parameters::default(),
        }
    }

    pub fn congress(mut self, congress: u32) -> Self {
        self.congress = Some(congress);
        self
    }

    pub fn amendment_type(mut self, amendment_type: AmendmentType) -> Self {
        self.amendment_type = Some(amendment_type);
        self
    }

    pub async fn send(&self) -> Result<AmendmentsResponse> {
        let mut path = String::from("/v3/amendment");
        if let Some(congress) = self.congress {
            path.push_str(format!("/{congress}").as_str());
        }
        if let Some(amendment_type) = &self.amendment_type {
            path.push_str(format!("/{amendment_type}").as_str());
        }
        self.client.get(&path, Some(&self.params)).await
    }
}

use super::BillHandler;
use crate::{
    chamber::Chamber,
    pagination::Pagination,
    parameters::{HasParameters, PageParameters, Parameters},
    Result,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Activity {
    pub date: DateTime<Utc>,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub enum CommitteeType {
    Standing,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Committee {
    pub activities: Vec<Activity>,
    pub chamber: Chamber,
    pub name: String,
    pub system_code: String,
    #[serde(rename = "type")]
    pub committee_type: CommitteeType,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitteesResponse {
    pub committees: Vec<Committee>,
    pagination: Pagination,
}

#[derive(Debug)]
pub struct CommitteesHandler<'client> {
    handler: &'client BillHandler<'client>,
    parameters: Parameters,
}

impl<'client> CommitteesHandler<'client> {
    pub(super) fn new(handler: &'client BillHandler) -> Self {
        CommitteesHandler {
            handler,
            parameters: Parameters::default(),
        }
    }

    pub async fn send(&self) -> Result<CommitteesResponse> {
        let path = format!(
            "/v3/bill/{congress}/{bill_type}/{bill_number}/committees",
            congress = self.handler.congress,
            bill_type = self.handler.bill_type,
            bill_number = self.handler.bill_number
        );

        self.handler.client.get(&path, Some(&self.parameters)).await
    }
}

impl<'client> HasParameters for CommitteesHandler<'client> {
    fn get_parameters(&mut self) -> &mut Parameters {
        &mut self.parameters
    }
}

impl<'client> crate::parameters::private::Sealed for CommitteesHandler<'client> {}

impl<'client> PageParameters for CommitteesHandler<'client> {}

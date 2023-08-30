use super::BillHandler;
use crate::{
    parameters::{HasParameters, PageParameters, Parameters},
    Result,
};
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Committee {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitteesResponse {
    pub committees: Vec<Committee>,
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

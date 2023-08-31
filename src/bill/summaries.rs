use super::BillHandler;
use crate::{
    pagination::Pagination,
    parameters::{HasParameters, PageParameters, Parameters},
    Result,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub action_date: NaiveDate,
    pub action_desc: String,
    pub text: String,
    pub update_date: DateTime<Utc>,
    pub version_code: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummariesResponse {
    pub summaries: Vec<Summary>,
    pagination: Pagination,
}

#[derive(Debug)]
pub struct SummariesHandler<'client> {
    handler: &'client BillHandler<'client>,
    parameters: Parameters,
}

impl<'client> SummariesHandler<'client> {
    pub(super) fn new(handler: &'client BillHandler) -> Self {
        SummariesHandler {
            handler,
            parameters: Parameters::default(),
        }
    }

    pub async fn send(&self) -> Result<SummariesResponse> {
        let path = format!(
            "/v3/bill/{congress}/{bill_type}/{bill_number}/summaries",
            congress = self.handler.congress,
            bill_type = self.handler.bill_type,
            bill_number = self.handler.bill_number
        );

        self.handler.client.get(&path, Some(&self.parameters)).await
    }
}

impl<'client> HasParameters for SummariesHandler<'client> {
    fn get_parameters(&mut self) -> &mut Parameters {
        &mut self.parameters
    }
}

impl<'client> crate::parameters::private::Sealed for SummariesHandler<'client> {}

impl<'client> PageParameters for SummariesHandler<'client> {}

use super::BillHandler;
use crate::{
    parameters::{HasParameters, PageParameters, Parameters},
    Result,
};
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedBill {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedBillsResponse {
    pub related_bills: Vec<RelatedBill>,
}

#[derive(Debug)]
pub struct RelatedBillsHandler<'client> {
    handler: &'client BillHandler<'client>,
    parameters: Parameters,
}

impl<'client> RelatedBillsHandler<'client> {
    pub(super) fn new(handler: &'client BillHandler) -> Self {
        RelatedBillsHandler {
            handler,
            parameters: Parameters::default(),
        }
    }

    pub async fn send(&self) -> Result<RelatedBillsResponse> {
        let path = format!(
            "/v3/bill/{congress}/{bill_type}/{bill_number}/relatedbills",
            congress = self.handler.congress,
            bill_type = self.handler.bill_type,
            bill_number = self.handler.bill_number
        );

        self.handler.client.get(&path, Some(&self.parameters)).await
    }
}

impl<'client> HasParameters for RelatedBillsHandler<'client> {
    fn get_parameters(&mut self) -> &mut Parameters {
        &mut self.parameters
    }
}

impl<'client> crate::parameters::private::Sealed for RelatedBillsHandler<'client> {}

impl<'client> PageParameters for RelatedBillsHandler<'client> {}

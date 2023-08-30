use super::BillHandler;
use crate::{
    parameters::{HasParameters, PageParameters, Parameters},
    Result,
};
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amendment {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendmentsResponse {
    pub amendments: Vec<Amendment>,
}

#[derive(Debug)]
pub struct AmendmentsHandler<'client> {
    handler: &'client BillHandler<'client>,
    parameters: Parameters,
}

impl<'client> AmendmentsHandler<'client> {
    pub(super) fn new(handler: &'client BillHandler) -> Self {
        AmendmentsHandler {
            handler,
            parameters: Parameters::default(),
        }
    }

    pub async fn send(&self) -> Result<AmendmentsResponse> {
        let path = format!(
            "/v3/bill/{congress}/{bill_type}/{bill_number}/amendments",
            congress = self.handler.congress,
            bill_type = self.handler.bill_type,
            bill_number = self.handler.bill_number
        );

        self.handler.client.get(&path, Some(&self.parameters)).await
    }
}

impl<'client> HasParameters for AmendmentsHandler<'client> {
    fn get_parameters(&mut self) -> &mut Parameters {
        &mut self.parameters
    }
}

impl<'client> crate::parameters::private::Sealed for AmendmentsHandler<'client> {}

impl<'client> PageParameters for AmendmentsHandler<'client> {}

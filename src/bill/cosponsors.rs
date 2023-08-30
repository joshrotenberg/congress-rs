use super::BillHandler;
use crate::{
    parameters::{HasParameters, PageParameters, Parameters},
    Result,
};
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cosponsor {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosponsorsResponse {
    pub cosponsors: Vec<Cosponsor>,
}

#[derive(Debug)]
pub struct CosponsorsHandler<'client> {
    handler: &'client BillHandler<'client>,
    parameters: Parameters,
}

impl<'client> CosponsorsHandler<'client> {
    pub(super) fn new(handler: &'client BillHandler) -> Self {
        CosponsorsHandler {
            handler,
            parameters: Parameters::default(),
        }
    }

    pub async fn send(&self) -> Result<CosponsorsResponse> {
        let path = format!(
            "/v3/bill/{congress}/{bill_type}/{bill_number}/cosponsors",
            congress = self.handler.congress,
            bill_type = self.handler.bill_type,
            bill_number = self.handler.bill_number
        );

        self.handler.client.get(&path, Some(&self.parameters)).await
    }
}

impl<'client> HasParameters for CosponsorsHandler<'client> {
    fn get_parameters(&mut self) -> &mut Parameters {
        &mut self.parameters
    }
}

impl<'client> crate::parameters::private::Sealed for CosponsorsHandler<'client> {}

impl<'client> PageParameters for CosponsorsHandler<'client> {}

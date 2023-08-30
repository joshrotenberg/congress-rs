use super::BillHandler;
use crate::{
    parameters::{HasParameters, PageParameters, Parameters},
    Result,
};
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitlesResponse {}

#[derive(Debug)]
pub struct TitlesHandler<'client> {
    handler: &'client BillHandler<'client>,
    parameters: Parameters,
}

impl<'client> TitlesHandler<'client> {
    pub(super) fn new(handler: &'client BillHandler) -> Self {
        TitlesHandler {
            handler,
            parameters: Parameters::default(),
        }
    }

    pub async fn send(&self) -> Result<TitlesResponse> {
        let path = format!(
            "/v3/bill/{congress}/{bill_type}/{bill_number}/titles",
            congress = self.handler.congress,
            bill_type = self.handler.bill_type,
            bill_number = self.handler.bill_number
        );

        self.handler.client.get(&path, Some(&self.parameters)).await
    }
}

impl<'client> HasParameters for TitlesHandler<'client> {
    fn get_parameters(&mut self) -> &mut Parameters {
        &mut self.parameters
    }
}

impl<'client> crate::parameters::private::Sealed for TitlesHandler<'client> {}

impl<'client> PageParameters for TitlesHandler<'client> {}

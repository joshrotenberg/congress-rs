use super::BillHandler;
use crate::{
    parameters::{HasParameters, PageParameters, Parameters},
    Result,
};
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextResponse {}

#[derive(Debug)]
pub struct TextHandler<'client> {
    handler: &'client BillHandler<'client>,
    parameters: Parameters,
}

impl<'client> TextHandler<'client> {
    pub(super) fn new(handler: &'client BillHandler) -> Self {
        TextHandler {
            handler,
            parameters: Parameters::default(),
        }
    }

    pub async fn send(&self) -> Result<TextResponse> {
        let path = format!(
            "/v3/bill/{congress}/{bill_type}/{bill_number}/text",
            congress = self.handler.congress,
            bill_type = self.handler.bill_type,
            bill_number = self.handler.bill_number
        );

        self.handler.client.get(&path, Some(&self.parameters)).await
    }
}

impl<'client> HasParameters for TextHandler<'client> {
    fn get_parameters(&mut self) -> &mut Parameters {
        &mut self.parameters
    }
}

impl<'client> crate::parameters::private::Sealed for TextHandler<'client> {}

impl<'client> PageParameters for TextHandler<'client> {}

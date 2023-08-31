use super::BillHandler;
use crate::{
    pagination::Pagination,
    parameters::{HasParameters, PageParameters, Parameters},
    Result,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    #[serde(rename = "type")]
    pub type_: String,
    url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextVersion {
    pub date: DateTime<Utc>,
    pub formats: Vec<Format>,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextResponse {
    pub text_versions: Vec<TextVersion>,
    pagination: Pagination,
}

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

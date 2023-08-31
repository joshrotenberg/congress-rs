use super::BillHandler;
use crate::{
    chamber::{Chamber, ChamberCode},
    pagination::Pagination,
    parameters::{HasParameters, PageParameters, Parameters},
    Result,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub bill_text_version_code: String,
    pub bill_text_version_name: String,
    pub chamber_code: ChamberCode,
    pub chamber_name: Chamber,
    pub title: String,
    pub title_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitlesResponse {
    pub titles: Vec<Title>,
    pagination: Pagination,
}

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

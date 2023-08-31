use super::BillHandler;
use crate::{
    amendment_type::AmendmentType,
    latest_action::LatestAction,
    pagination::Pagination,
    parameters::{HasParameters, PageParameters, Parameters},
    Result,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amendment {
    pub congress: u32,
    pub description: String,
    pub latest_action: LatestAction,
    pub number: String,
    #[serde(rename = "type")]
    pub amendment_type: AmendmentType,
    pub update_date: DateTime<Utc>,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendmentsResponse {
    pub amendments: Vec<Amendment>,
    pagination: Pagination,
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

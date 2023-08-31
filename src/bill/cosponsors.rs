use super::BillHandler;
use crate::{
    pagination::Pagination,
    parameters::{HasParameters, PageParameters, Parameters},
    Result,
};
use chrono::NaiveDate;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cosponsor {
    pub bioguide_id: String,
    pub district: u32,
    pub first_name: String,
    pub full_name: String,
    pub is_originial_cosponsor: bool,
    pub last_name: String,
    pub middle_name: String,
    pub party: String,
    pub sponsorship_date: NaiveDate,
    pub state: String,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosponsorsResponse {
    pub cosponsors: Vec<Cosponsor>,
    pagination: Pagination,
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

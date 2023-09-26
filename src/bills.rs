use crate::{
    bill_type::BillType,
    chamber::{Chamber, ChamberCode},
    latest_action::LatestAction,
    pagination::Pagination,
    parameters::{HasParameters, PageParameters, Parameters, SortParameters},
    Client, Result,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use url::Url;

// Types
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bill {
    pub congress: u32,
    pub latest_action: LatestAction,
    pub number: String,
    pub origin_chamber: Chamber,
    pub origin_chamber_code: ChamberCode,
    pub title: String,
    #[serde(rename = "type")]
    pub bill_type: BillType,
    pub update_date: NaiveDate,
    pub update_date_including_text: DateTime<Utc>,
    pub url: Url,
}

// Response
#[derive(Debug, Deserialize)]
pub struct BillsResponse {
    pub bills: Vec<Bill>,
    pagination: Pagination,
}

crate::pagination::macros::paged_iterator!(BillsResponse, Bill, bills);

// Handler
#[derive(Debug)]
pub struct BillsHandler<'client> {
    client: &'client Client,
    congress: Option<u32>,
    bill_type: Option<BillType>,
    parameters: Parameters,
}

impl<'client> BillsHandler<'client> {
    pub(super) fn new(client: &'client Client) -> Self {
        BillsHandler {
            client,
            congress: None,
            bill_type: None,
            parameters: Parameters::default(),
        }
    }

    pub fn congress(mut self, congress: u32) -> Self {
        self.congress = Some(congress);
        self
    }

    pub fn bill_type(mut self, bill_type: BillType) -> Self {
        self.bill_type = Some(bill_type);
        self
    }

    pub async fn send(&self) -> Result<BillsResponse> {
        let mut path = String::from("/v3/bill");
        if let Some(congress) = self.congress {
            path.push_str(format!("/{congress}").as_str());
        }
        if let Some(bill_type) = &self.bill_type {
            path.push_str(format!("/{bill_type}").as_str());
        }
        self.client.get(&path, Some(&self.parameters)).await
    }
}

impl<'client> HasParameters for BillsHandler<'client> {
    fn get_parameters(&mut self) -> &mut Parameters {
        &mut self.parameters
    }
}

impl<'client> crate::parameters::private::Sealed for BillsHandler<'client> {}

impl<'client> PageParameters for BillsHandler<'client> {}

impl<'client> SortParameters for BillsHandler<'client> {}

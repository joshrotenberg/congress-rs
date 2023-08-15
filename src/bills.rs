use crate::{
    amendment_type::{self, AmendmentType},
    bill_type::{self, BillType},
    chamber::Chamber,
    page::Pagination,
    parameters::Parameters,
    Client, Result,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct LatestAction {
    pub action_date: NaiveDate,
    pub text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bill {
    pub congress: u32,
    pub latest_action: LatestAction,
    pub number: String,
    pub origin_chamber: Chamber,
    pub origin_chamber_code: String,
    pub title: String,
    pub bill_type: BillType,
    pub update_date: NaiveDate,
    pub update_date_including_text: DateTime<Utc>,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
pub struct BillsResponse {
    pub bills: Vec<Bill>,
    pub pagination: Pagination,
}

#[derive(Debug)]
pub struct BillsHandler<'client> {
    client: &'client Client,
    congress: Option<u32>,
    bill_type: Option<BillType>,
    params: Parameters,
}

impl<'client> BillsHandler<'client> {
    pub fn new(client: &'client Client) -> Self {
        BillsHandler {
            client,
            congress: None,
            bill_type: None,
            params: Parameters::default(),
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
        let mut path = String::from("/v3/amendment");
        if let Some(congress) = self.congress {
            path.push_str(format!("/{congress}").as_str());
        }
        if let Some(bill_type) = &self.bill_type {
            path.push_str(format!("/{bill_type}").as_str());
        }
        self.client.get(&path, Some(&self.params)).await
    }
}

crate::parameters::macros::implement_page_params!(BillsHandler);
crate::parameters::macros::implement_sort_params!(BillsHandler);
crate::page::macros::implement_paged_response!(BillsResponse, Bill, bills);

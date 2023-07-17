use crate::{
    bill_type::BillType, page::Pagination, parameters::Parameters, Client, PagedResponse, Result,
};
use serde::Deserialize;
use std::slice::Iter;
use url::Url;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Bill {
    pub congress: u32,
}

#[derive(Debug, Deserialize)]
pub struct BillsResponse {
    bills: Vec<Bill>,
    pagination: Pagination,
}

#[derive(Debug)]
pub struct BillsHandler<'client> {
    client: &'client Client,
    congress: Option<u32>,
    bill_type: Option<BillType>,
    params: Parameters,
}

impl<'client> BillsHandler<'client> {
    pub fn new(client: &'client Client) -> BillsHandler<'client> {
        BillsHandler {
            client: client,
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

    pub async fn bills(&self) -> Result<BillsResponse> {
        let mut path = String::from("/v3/bill");
        if let Some(congress) = self.congress {
            path.push_str(format!("/{congress}").as_str());
        }
        if let Some(bill_type) = &self.bill_type {
            path.push_str(format!("/{bill_type}").as_str());
        }
        Ok(self.client.get(&path, Some(&self.params)).await?)
    }
}

crate::parameters::macros::implement_page_params!(BillsHandler);
crate::page::macros::implement_paged_response!(BillsResponse, Bill, bills);

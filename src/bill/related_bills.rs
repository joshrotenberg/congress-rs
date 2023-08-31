use super::BillHandler;
use crate::{
    bill_type::BillType,
    latest_action::LatestAction,
    pagination::Pagination,
    parameters::{HasParameters, PageParameters, Parameters},
    Result,
};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipDetail {
    pub identified_by: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedBill {
    pub congress: u32,
    pub latest_action: LatestAction,
    pub number: u32,
    pub relationship_details: Vec<RelationshipDetail>,
    pub title: String,
    #[serde(rename = "type")]
    pub bill_type: BillType,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedBillsResponse {
    pub related_bills: Vec<RelatedBill>,
    pub(crate) pagination: Pagination,
}

#[derive(Debug)]
pub struct RelatedBillsHandler<'client> {
    handler: &'client BillHandler<'client>,
    parameters: Parameters,
}

impl<'client> RelatedBillsHandler<'client> {
    pub(super) fn new(handler: &'client BillHandler) -> Self {
        RelatedBillsHandler {
            handler,
            parameters: Parameters::default(),
        }
    }

    pub async fn send(&self) -> Result<RelatedBillsResponse> {
        let path = format!(
            "/v3/bill/{congress}/{bill_type}/{bill_number}/relatedbills",
            congress = self.handler.congress,
            bill_type = self.handler.bill_type,
            bill_number = self.handler.bill_number
        );

        self.handler.client.get(&path, Some(&self.parameters)).await
    }
}

impl<'client> HasParameters for RelatedBillsHandler<'client> {
    fn get_parameters(&mut self) -> &mut Parameters {
        &mut self.parameters
    }
}

impl<'client> crate::parameters::private::Sealed for RelatedBillsHandler<'client> {}

impl<'client> PageParameters for RelatedBillsHandler<'client> {}

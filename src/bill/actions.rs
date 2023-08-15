use super::BillHandler;
use crate::{page::Pagination, Result};
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SourceSystem {
    pub code: u32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ActionType {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub action_code: String,
    pub action_date: NaiveDate,
    pub source_system: SourceSystem,
    pub text: String,
    pub action_type: ActionType,
}

#[derive(Debug, Deserialize)]
pub struct ActionsResponse {
    pub actions: Vec<Action>,
    pub pagination: Pagination,
}

#[derive(Debug)]
pub struct ActionsHandler<'client> {
    handler: &'client BillHandler<'client>,
}

impl<'client> ActionsHandler<'client> {
    pub(super) fn new(handler: &'client BillHandler) -> Self {
        Self { handler }
    }

    pub async fn send(&self) -> Result<ActionsResponse> {
        let path = format!(
            "/v3/bill/{}/{}/{}/actions",
            self.handler.congress, self.handler.bill_type, self.handler.bill_number
        );

        self.handler
            .client
            .get(&path, Some(&self.handler.params))
            .await
    }
}

crate::parameters::macros::implement_page_params!(ActionsHandler);
crate::page::macros::implement_paged_response!(ActionsResponse, Action, actions);

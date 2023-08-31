use super::BillHandler;
use crate::{
    pagination::Pagination,
    parameters::{HasParameters, PageParameters, Parameters},
    Result,
};
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SourceSystem {
    pub code: Option<u32>,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub enum ActionType {
    BecameLaw,
    President,
    Committee,
    IntroReferral,
    Calendars,
    Floor,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub action_code: Option<String>,
    pub action_date: NaiveDate,
    pub source_system: SourceSystem,
    pub text: String,
    #[serde(rename = "type")]
    pub action_type: ActionType,
}

#[derive(Debug, Deserialize)]
pub struct ActionsResponse {
    pub actions: Vec<Action>,
    pagination: Pagination,
}

#[derive(Debug)]
pub struct ActionsHandler<'client> {
    handler: &'client BillHandler<'client>,
    parameters: Parameters,
}

impl<'client> ActionsHandler<'client> {
    pub(super) fn new(handler: &'client BillHandler) -> Self {
        ActionsHandler {
            handler,
            parameters: Parameters::default(),
        }
    }

    pub async fn send(&self) -> Result<ActionsResponse> {
        let path = format!(
            "/v3/bill/{congress}/{bill_type}/{bill_number}/actions",
            congress = self.handler.congress,
            bill_type = self.handler.bill_type,
            bill_number = self.handler.bill_number
        );

        self.handler.client.get(&path, Some(&self.parameters)).await
    }
}

impl<'client> HasParameters for ActionsHandler<'client> {
    fn get_parameters(&mut self) -> &mut Parameters {
        &mut self.parameters
    }
}

impl<'client> crate::parameters::private::Sealed for ActionsHandler<'client> {}

impl<'client> PageParameters for ActionsHandler<'client> {}

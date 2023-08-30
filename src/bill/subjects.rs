use super::BillHandler;
use crate::{
    parameters::{HasParameters, PageParameters, Parameters},
    Result,
};
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegislativeSubject {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectsResponse {
    // pub Legislative_subjects: Vec<LegislativeSubject>,
}

#[derive(Debug)]
pub struct SubjectsHandler<'client> {
    handler: &'client BillHandler<'client>,
    parameters: Parameters,
}

impl<'client> SubjectsHandler<'client> {
    pub(super) fn new(handler: &'client BillHandler) -> Self {
        SubjectsHandler {
            handler,
            parameters: Parameters::default(),
        }
    }

    pub async fn send(&self) -> Result<SubjectsResponse> {
        let path = format!(
            "/v3/bill/{congress}/{bill_type}/{bill_number}/subjects",
            congress = self.handler.congress,
            bill_type = self.handler.bill_type,
            bill_number = self.handler.bill_number
        );

        self.handler.client.get(&path, Some(&self.parameters)).await
    }
}

impl<'client> HasParameters for SubjectsHandler<'client> {
    fn get_parameters(&mut self) -> &mut Parameters {
        &mut self.parameters
    }
}

impl<'client> crate::parameters::private::Sealed for SubjectsHandler<'client> {}

impl<'client> PageParameters for SubjectsHandler<'client> {}

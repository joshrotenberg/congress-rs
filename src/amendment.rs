use self::amendments::AmendmentsHandler;
use crate::{
    amendment::{actions::ActionsHandler, cosponsors::CosponsorsHandler},
    amendment_type::AmendmentType,
    bill_type::BillType,
    chamber::Chamber,
    parameters::Parameters,
    Client, Result,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use url::Url;

pub mod actions;
pub mod amendments;
pub mod cosponsors;

#[derive(Debug, Deserialize)]
pub struct Actions {
    pub count: u32,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendedBill {
    pub congress: u32,
    pub number: String,
    pub origin_chamber: Chamber,
    pub origin_chamber_code: String,
    pub title: String,
    #[serde(rename = "type")]
    pub bill_type: BillType,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
pub struct AmendmentsToAmendment {
    pub count: u32,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cosponsors {
    pub count: u32,
    pub count_including_withdrawn_cosponsors: u32,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestAction {
    pub action_date: NaiveDate,
    pub text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sponsor {
    pub bioguide_id: String,
    pub first_name: String,
    pub full_name: String,
    pub last_name: String,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amendment {
    pub actions: Actions,
    pub amended_bill: AmendedBill,
    pub amendments_to_amendment: AmendmentsToAmendment,
    pub chamber: Chamber,
    pub congress: u32,
    pub cosponsors: Cosponsors,
    pub latest_action: LatestAction,
    pub number: String,
    pub proposed_date: DateTime<Utc>,
    pub purpose: String,
    pub sponsors: Vec<Sponsor>,
    pub submitted_date: DateTime<Utc>,
    pub amendment_type: Option<AmendmentType>,
    pub update_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AmendmentResponse {
    pub amendment: Amendment,
}

#[derive(Debug)]
pub struct AmendmentHandler<'client> {
    client: &'client Client,
    congress: u32,
    amendment_type: AmendmentType,
    amendment_number: u32,
    params: Parameters,
}

impl<'client> AmendmentHandler<'client> {
    pub fn new(
        client: &'client Client,
        congress: impl Into<u32>,
        amendment_type: AmendmentType,
        amendment_number: impl Into<u32>,
    ) -> Self {
        AmendmentHandler {
            client,
            congress: congress.into(),
            amendment_type,
            amendment_number: amendment_number.into(),
            params: Parameters::default(),
        }
    }

    pub async fn send(&self) -> Result<AmendmentResponse> {
        let path = format!(
            "/v3/amendment/{}/{}/{}",
            self.congress, self.amendment_type, self.amendment_number
        );

        self.client.get(&path, Some(&self.params)).await
    }

    pub fn actions(&self) -> ActionsHandler {
        ActionsHandler::new(&self)
    }

    pub fn cosponsors(&self) -> CosponsorsHandler {
        CosponsorsHandler::new(&self)
    }

    pub fn amendments(&self) -> AmendmentsHandler {
        AmendmentsHandler::new(&self)
    }
}

use crate::{
    amendment_type::AmendmentType, bill_type::BillType, chamber::Chamber, parameters::Parameters,
    Client, Result,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use url::Url;

use self::{actions::ActionsHandler, amendments::AmendmentsHandler};

pub mod actions;
pub mod amendments;
pub mod committees;
pub mod cosponsors;
pub mod related_bills;
pub mod subjects;
pub mod summaries;
pub mod text;
pub mod titles;

#[derive(Debug, Deserialize)]
pub struct Actions {
    pub count: u32,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
pub struct Amendments {
    pub count: u32,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CboCostEstimate {
    pub description: String,
    pub pub_date: DateTime<Utc>,
    pub title: String,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
pub struct CommitteeReport {
    pub citation: String,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
pub struct Committees {
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
pub struct Law {
    pub number: String,
    #[serde(rename = "type")]
    pub law_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestAction {
    pub action_date: NaiveDate,
    pub text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyArea {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct RelatedBills {
    pub count: u32,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sponsor {
    pub bioguide_id: String,
    pub district: u32,
    pub first_name: String,
    pub full_name: String,
    pub is_by_request: String,
    pub last_name: String,
    pub middle_name: String,
    pub party: String,
    pub state: String,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
pub struct Subjects {
    pub count: u32,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
pub struct Summaries {
    pub count: u32,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
pub struct TextVersion {
    pub count: u32,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
pub struct Titles {
    pub count: u32,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bill {
    pub actions: Actions,
    pub amendments: Amendments,
    pub cbo_cost_estimates: Vec<CboCostEstimate>,
    pub committee_reports: Vec<CommitteeReport>,
    pub committees: Committees,
    pub congress: u32,
    pub constitutional_authority_statement_text: String,
    pub cosponsors: Cosponsors,
    pub introduced_date: NaiveDate,
    pub latest_action: LatestAction,
    pub laws: Vec<Law>,
    pub number: String,
    pub origin_chamber: Chamber,
    pub policy_area: PolicyArea,
    pub related_bills: RelatedBills,
    pub sponsors: Vec<Sponsor>,
    pub subjects: Subjects,
    pub summaries: Summaries,
    pub text_versions: TextVersion,
    pub title: String,
    pub titles: Titles,
    pub bill_type: BillType,
    pub update_date: DateTime<Utc>,
    pub update_date_including_text: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct BillResponse {
    pub bill: Bill,
}

#[derive(Debug)]
pub struct BillHandler<'client> {
    client: &'client Client,
    congress: u32,
    bill_type: BillType,
    bill_number: u32,
    params: Parameters,
}

impl<'client> BillHandler<'client> {
    pub fn new(
        client: &'client Client,
        congress: impl Into<u32>,
        bill_type: BillType,
        bill_number: impl Into<u32>,
    ) -> Self {
        BillHandler {
            client,
            congress: congress.into(),
            bill_type,
            bill_number: bill_number.into(),
            params: Parameters::default(),
        }
    }

    pub async fn send(&self) -> Result<BillResponse> {
        let path = format!(
            "/v3/bill/{}/{}/{}",
            self.congress, self.bill_type, self.bill_number
        );

        self.client.get(&path, Some(&self.params)).await
    }

    pub fn actions(&self) -> ActionsHandler {
        ActionsHandler::new(&self)
    }

    pub fn amendments(&self) -> AmendmentsHandler {
        AmendmentsHandler::new(&self)
    }
}

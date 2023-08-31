use crate::{bill_type::BillType, chamber::Chamber, latest_action::LatestAction, Client, Result};
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use url::Url;

use self::actions::ActionsHandler;
use self::amendments::AmendmentsHandler;
use self::committees::CommitteesHandler;
use self::cosponsors::CosponsorsHandler;
use self::related_bills::RelatedBillsHandler;
use self::subjects::SubjectsHandler;
use self::summaries::SummariesHandler;
use self::text::TextHandler;
use self::titles::TitlesHandler;

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
pub struct CboCostEstimate {
    pub description: String,
    pub pub_date: Option<DateTime<Utc>>,
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
pub struct Cosponsors {
    pub count: u32,
    pub count_including_withdrawn_cosponsors: Option<u32>,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
pub struct Law {
    pub number: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Deserialize)]
pub struct PolicyArea {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedBills {
    pub count: u32,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sponsor {
    pub bioguide_id: String,
    pub district: Option<u32>,
    pub first_name: String,
    pub full_name: String,
    pub is_by_request: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub party: String,
    pub state: String,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subjects {
    pub count: u32,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summaries {
    pub count: u32,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextVersions {
    pub count: u32,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Titles {
    pub count: u32,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bill {
    pub actions: Actions,
    pub amendments: Option<Amendments>,
    pub cbo_cost_estimates: Option<Vec<CboCostEstimate>>,
    pub committee_reports: Option<Vec<CommitteeReport>>,
    pub committees: Committees,
    pub congress: u32,
    pub constitutional_authority_statement_text: Option<String>,
    pub cosponsors: Option<Cosponsors>,
    pub introduced_date: NaiveDate,
    pub latest_action: LatestAction,
    pub laws: Option<Vec<Law>>,
    pub number: String,
    pub origin_chamber: Chamber,
    pub policy_area: Option<PolicyArea>,
    pub related_bills: Option<RelatedBills>,
    pub sponsors: Option<Vec<Sponsor>>,
    pub subjects: Option<Subjects>,
    pub summaries: Option<Summaries>,
    pub text_versions: Option<TextVersions>,
    pub title: String,
    pub update_date: DateTime<Utc>,
    pub update_date_including_text: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillResponse {
    pub bill: Bill,
}

#[derive(Debug)]
pub struct BillHandler<'client> {
    client: &'client Client,
    congress: u32,
    bill_type: BillType,
    bill_number: u32,
}

impl<'client> BillHandler<'client> {
    pub(super) fn new(
        client: &'client Client,
        congress: u32,
        bill_type: BillType,
        bill_number: u32,
    ) -> Self {
        BillHandler {
            client,
            congress,
            bill_type,
            bill_number,
        }
    }

    pub async fn send(&self) -> Result<BillResponse> {
        let path = format!(
            "/v3/bill/{congress}/{bill_type}/{bill_number}",
            congress = self.congress,
            bill_type = self.bill_type,
            bill_number = self.bill_number
        );

        self.client.get(&path, None::<&()>).await
    }

    pub fn actions(&self) -> ActionsHandler {
        ActionsHandler::new(self)
    }

    pub fn amendments(&self) -> AmendmentsHandler {
        AmendmentsHandler::new(self)
    }

    pub fn committees(&self) -> CommitteesHandler {
        CommitteesHandler::new(self)
    }

    pub fn cosponsors(&self) -> CosponsorsHandler {
        CosponsorsHandler::new(self)
    }

    pub fn related_bills(&self) -> RelatedBillsHandler {
        RelatedBillsHandler::new(self)
    }

    pub fn subjects(&self) -> SubjectsHandler {
        SubjectsHandler::new(self)
    }

    pub fn summaries(&self) -> SummariesHandler {
        SummariesHandler::new(self)
    }

    pub fn text(&self) -> TextHandler {
        TextHandler::new(self)
    }

    pub fn titles(&self) -> TitlesHandler {
        TitlesHandler::new(self)
    }
}

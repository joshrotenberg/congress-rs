#![doc = include_str!("../README.md")]
use bill::BillHandler;
use bill_type::BillType;
use bills::BillsHandler;
use error::{
    ClientBuildSnafu, InvalidBaseUrlSnafu, InvalidUrlSnafu, JsonParseSnafu, ParameterSnafu,
    ResponseSnafu, SendSnafu,
};
use pagination::PagedResponse;
use parameters::Parameters;
use reqwest::IntoUrl;
use serde::Serialize;
use snafu::ResultExt;
use std::fmt::Debug;
use url::Url;

pub use error::Result;

pub mod amendment_type;
pub mod bill_type;
pub mod chamber;
pub mod error;
pub mod latest_action;
pub mod pagination;
pub mod parameters;
pub mod sort;

pub mod bill;
pub mod bills;

static DEFAULT_BASE_URL: &str = "https://api.congress.gov/";
static DEFAULT_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    base_url: Url,
    api_key: String,
}

impl Client {
    pub fn bills(&self) -> BillsHandler {
        BillsHandler::new(self)
    }

    pub fn bill(&self, congress: u32, bill_type: BillType, bill_number: u32) -> BillHandler {
        BillHandler::new(self, congress, bill_type, bill_number)
    }

    pub async fn previous<T, R>(&self, response: &T) -> Result<Option<T>>
    where
        T: serde::de::DeserializeOwned + PagedResponse<R>,
    {
        self.get_page(response.previous()).await
    }

    pub async fn next<T, R>(&self, response: &T) -> Result<Option<T>>
    where
        T: serde::de::DeserializeOwned + PagedResponse<R>,
    {
        self.get_page(response.next()).await
    }

    async fn get_page<T, R>(&self, url: Option<Url>) -> Result<Option<T>>
    where
        T: serde::de::DeserializeOwned + PagedResponse<R>,
    {
        if let Some(url) = url {
            let params: Parameters = url
                .query()
                .expect("Url doesn't have a query'")
                .parse::<Parameters>()
                .context(ParameterSnafu)?;
            let page: T = self.get(url.path(), Some(&params)).await?;
            Ok(Some(page))
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn get<R, P>(&self, path: &str, params: Option<&P>) -> Result<R>
    where
        P: Serialize + ?Sized + Debug,
        R: serde::de::DeserializeOwned,
    {
        let url = self.base_url.join(path).context(InvalidUrlSnafu)?;
        let mut request = self.client.get(url).query(&[
            ("api_key", &self.api_key),
            ("format", &String::from("json")),
        ]);
        if let Some(p) = params {
            request = request.query(p);
        }

        let response = request.send().await.context(SendSnafu)?;
        let is_success = response.status().is_success();
        let text = response.text().await.context(ResponseSnafu)?;
        let de = &mut serde_json::Deserializer::from_str(&text);

        if is_success {
            Ok(serde_path_to_error::deserialize(de).context(JsonParseSnafu)?)
        } else {
            Err(crate::error::Error::Congress {
                source: serde_path_to_error::deserialize(de).context(JsonParseSnafu)?,
            })
        }
    }
}

#[derive(Debug)]
pub struct ClientBuilder {
    base_url: Url,
    user_agent: String,
    api_key: String,
}

impl ClientBuilder {
    pub fn new(api_key: impl Into<String>) -> ClientBuilder {
        ClientBuilder {
            base_url: Url::parse(DEFAULT_BASE_URL).unwrap(),
            user_agent: DEFAULT_USER_AGENT.into(),
            api_key: api_key.into(),
        }
    }

    pub fn base_url(mut self, base_url: impl IntoUrl) -> Result<Self> {
        self.base_url = base_url.into_url().context(InvalidBaseUrlSnafu)?;
        Ok(self)
    }

    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    pub fn build(&self) -> Result<Client> {
        let client = reqwest::ClientBuilder::new()
            .user_agent(self.user_agent.clone())
            .build()
            .context(ClientBuildSnafu)?;

        Ok(Client {
            client,
            base_url: self.base_url.clone(),
            api_key: self.api_key.clone(),
        })
    }
}

pub mod prelude {
    pub use crate::parameters::PageParameters;
    pub use crate::parameters::SortParameters;

    pub use crate::sort::Sort;
}

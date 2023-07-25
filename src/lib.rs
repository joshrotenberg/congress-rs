//! # Congress
//!
//! `congress-rs` is a client for the United States Congress API: <https://api.congress.gov>
//!
//!
use bills::BillsHandler;
use error::{ClientSnafu, InvalidBaseUrlSnafu, InvalidUrlSnafu, JsonPathToSnafu};
use page::PagedResponse;
use parameters::Parameters;
use reqwest::IntoUrl;
use serde::{self, Serialize};
use snafu::ResultExt;
use std::fmt::Debug;
use std::marker::PhantomData;
use url::Url;

pub use error::Result;

pub mod bill_type;
pub mod error;
pub mod page;
pub mod parameters;

pub mod amendments;
pub mod bills;
pub mod committee_meetings;
pub mod committee_prints;
pub mod committee_reports;
pub mod committees;
pub mod congresses;
pub mod congressional_records;
pub mod hearings;
pub mod house_communications;
pub mod house_requirements;
pub mod members;
pub mod nominations;
pub mod senate_communications;
pub mod summaries;
pub mod treaties;

static DEFAULT_BASE_URL: &str = "https://api.congress.gov/";
static DEFAULT_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// A Client builder.
#[derive(Debug)]
pub struct ClientBuilder<State> {
    base_url: Url,
    user_agent: String,
    api_key: Option<String>,
    state: PhantomData<State>,
}

impl Default for ClientBuilder<WithoutApiKey> {
    fn default() -> ClientBuilder<WithoutApiKey> {
        ClientBuilder {
            base_url: Url::parse(DEFAULT_BASE_URL)
                .context(InvalidUrlSnafu)
                .unwrap(),
            user_agent: DEFAULT_USER_AGENT.into(),
            api_key: None,
            state: PhantomData,
        }
    }
}

pub struct WithApiKey;
pub struct WithoutApiKey;

impl ClientBuilder<WithoutApiKey> {
    /// Get a new, default client.
    /// ```no_run
    /// # use congress::{bills::BillsResponse, Client, ClientBuilder};
    /// # async fn run() -> congress::Result<()> {
    /// # let client = ClientBuilder::new().api_key("test").build()?;
    /// let builder = ClientBuilder::new();
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> ClientBuilder<WithoutApiKey> {
        ClientBuilder::default()
    }

    /// Change the API base url from the default.
    /// ```no_run
    /// # use congress::{bills::BillsResponse, Client, ClientBuilder};
    /// # async fn run() -> congress::Result<()> {
    /// # let client = ClientBuilder::new().api_key("test").build()?;
    /// let builder = ClientBuilder::new().base_url("http://i.know.what.im.doing.com");
    /// # Ok(())
    /// # }
    /// ```
    pub fn base_url(mut self, base_url: impl IntoUrl) -> Result<Self> {
        self.base_url = base_url.into_url().context(InvalidBaseUrlSnafu)?;
        Ok(self)
    }

    /// Change the client user agent from the default.
    /// ```no_run
    /// # use congress::{bills::BillsResponse, Client, ClientBuilder};
    /// # async fn run() -> congress::Result<()> {
    /// # let client = ClientBuilder::new().api_key("test").build()?;
    /// let builder = ClientBuilder::new().user_agent("my-special-client/0.1.2");
    /// # Ok(())
    /// # }
    /// ```
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// Set the api.congress.gov API key. An API key is required to call the `build` method.
    /// ```no_run
    /// # use congress::{bills::BillsResponse, Client, ClientBuilder};
    /// # async fn run() -> congress::Result<()> {
    /// # let client = ClientBuilder::new().api_key("test").build()?;
    /// let builder = ClientBuilder::new().api_key("my-secret-key");
    /// # Ok(())
    /// # }
    /// ```
    pub fn api_key(self, api_key: impl Into<String>) -> ClientBuilder<WithApiKey> {
        ClientBuilder {
            base_url: self.base_url,
            user_agent: self.user_agent,
            api_key: Some(api_key.into()),
            state: PhantomData,
        }
    }
}

impl ClientBuilder<WithApiKey> {
    /// Build a new Client instance.
    /// ```no_run
    /// # use congress::{bills::BillsResponse, Client, ClientBuilder};
    /// # async fn run() -> congress::Result<()> {
    /// # let client = ClientBuilder::new().api_key("test").build()?;
    /// let builder = ClientBuilder::new().api_key("my-secret-key").build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn build(&self) -> Result<Client> {
        let client = reqwest::ClientBuilder::new()
            .user_agent(self.user_agent.clone())
            .build()
            .context(ClientSnafu)?;

        Ok(Client {
            client,
            base_url: self.base_url.clone(),
            api_key: self.api_key.clone().unwrap(),
        })
    }
}

/// The client.
#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    base_url: Url,
    api_key: String,
}

impl Client {
    /// Get a new `ClientBuilder`.
    pub fn builder() -> ClientBuilder<WithoutApiKey> {
        ClientBuilder::default()
    }

    /// Get the most recent bills, or narrow the search by Congress and Bill type.
    /// ```no_run
    /// # use congress::{bills::BillsResponse, Client, ClientBuilder};
    /// # async fn run() -> congress::Result<()> {
    /// # let client = ClientBuilder::new().api_key("test").build()?;
    /// let bills: BillsResponse = client.bills().bills().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn bills(&self) -> BillsHandler {
        BillsHandler::new(self)
    }

    /// Get the previous page of the results for the given response, if any.
    /// ```no_run
    /// # use congress::{bills::BillsResponse, Client, ClientBuilder};
    /// # async fn run() -> congress::Result<()> {
    /// # let client = ClientBuilder::new().api_key("test").build()?;
    /// let bills: BillsResponse = client.bills().offset(2).bills().await?;
    /// let next: congress::Result<Option<BillsResponse>> = client.previous(&bills).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn previous<T>(&self, response: &T) -> Result<Option<T>>
    where
        T: serde::de::DeserializeOwned + PagedResponse,
    {
        match response.previous() {
            Some(url) => {
                let params: Parameters = url.query().unwrap().parse::<Parameters>().unwrap();
                let previous: T = self.get(url.path(), Some(&params)).await?;
                Ok(Some(previous))
            }
            _ => Ok(None),
        }
    }

    /// Get the next page of the results for the given response, if any.
    /// ```no_run
    /// # use congress::{bills::BillsResponse, Client, ClientBuilder};
    /// # async fn run() -> congress::Result<()> {
    /// # let client = ClientBuilder::new().api_key("test").build()?;
    /// let bills: BillsResponse = client.bills().bills().await?;
    /// let next: congress::Result<Option<BillsResponse>> = client.next(&bills).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn next<T>(&self, response: &T) -> Result<Option<T>>
    where
        T: serde::de::DeserializeOwned + PagedResponse,
    {
        match response.next() {
            Some(url) => {
                let params: Parameters = url.query().unwrap().parse::<Parameters>().unwrap();

                let next: T = self.get(url.path(), Some(&params)).await?;
                Ok(Some(next))
            }
            _ => Ok(None),
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

        let response = request.send().await.context(ClientSnafu)?;
        let is_success = response.status().is_success();
        let text = response.text().await.context(ClientSnafu)?;
        let de = &mut serde_json::Deserializer::from_str(&text);

        if is_success {
            Ok(serde_path_to_error::deserialize(de).context(JsonPathToSnafu)?)
        } else {
            Err(crate::error::Error::Congress {
                source: serde_path_to_error::deserialize(de).context(JsonPathToSnafu)?,
            })
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn default_builder() {
        ClientBuilder::default();
    }

    #[test]
    fn custom_builder() {
        assert!(ClientBuilder::new()
            .base_url("http://my.url.com")
            .unwrap()
            .user_agent("user/0.1.0")
            .api_key("bigsecret")
            .build()
            .is_ok());
    }

    #[test]
    fn bad_base_url() {
        assert!(ClientBuilder::new().base_url("ugh ugh ugh").is_err());
    }

    #[test]
    fn client() -> Result<()> {
        assert!(ClientBuilder::new().api_key("my key").build().is_ok());

        Ok(())
    }
}

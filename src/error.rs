use serde::Deserialize;
use snafu::Snafu;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct CongressError {
    pub error: String,
}

impl std::fmt::Display for CongressError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.error)?;
        Ok(())
    }
}

impl std::error::Error for CongressError {}

#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
pub enum Error {
    /// An API key cannot be empty
    #[snafu(display("API key can't be empty"))]
    EmptyApiKey {},

    /// The base URL is invalid
    #[snafu(display("Invalid Base URL"))]
    InvalidBaseUrl {
        /// The source error
        source: reqwest::Error,
    },

    /// The URL is invalid
    #[snafu(display("Invalid URI"))]
    InvalidUrl {
        /// The source error
        source: url::ParseError,
    },

    #[snafu(display("Error building HTTP client"))]
    ClientError {
        /// The source error
        source: reqwest::Error,
    },

    /// A JSON parsing error
    #[snafu(display("JSON Error: {}", source))]
    Json {
        /// The source error
        source: serde_json::Error,
    },

    /// A JSON parsing error
    #[snafu(display("JSON Error: {}", source))]
    JsonPathToError {
        /// The source error
        source: serde_path_to_error::Error<serde_json::Error>,
    },

    /// A URL Decoding error
    #[snafu(display("URL Decoding Error: {}", source))]
    UrlDecodingError { source: serde_urlencoded::de::Error },

    #[snafu(display("Congress API Error: {}", source))]
    Congress {
        /// The source error
        source: CongressError,
    },
}

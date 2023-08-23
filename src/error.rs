use serde::Deserialize;
use snafu::Snafu;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct CongressError {
    pub error: String,
}

impl std::error::Error for CongressError {}
impl std::fmt::Display for CongressError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.error)?;
        Ok(())
    }
}

#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
pub enum Error {
    /// Unable to build the client
    #[snafu(display("Unable to build the underlying client: {}", source))]
    ClientBuildError { source: reqwest::Error },

    /// The Base URL is invalid
    #[snafu(display("Invalid Base URL"))]
    InvalidBaseUrlError {
        /// The source error
        source: reqwest::Error,
    },

    /// The URL is invalid
    #[snafu(display("Invalid URL"))]
    InvalidUrl {
        /// The source error
        source: url::ParseError,
    },

    #[snafu(display("Error sending the HTTP request"))]
    SendError {
        /// The source error
        source: reqwest::Error,
    },

    #[snafu(display("Error processing the HTTP response"))]
    ResponseError {
        /// The source error
        source: reqwest::Error,
    },

    #[snafu(display("JSON Error: {}", source))]
    JsonParseError {
        /// The source error
        source: serde_path_to_error::Error<serde_json::Error>,
    },

    #[snafu(display("Congress API Error: {}", source))]
    Congress {
        /// The source error
        source: CongressError,
    },
}

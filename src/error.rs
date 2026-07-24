use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("API request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("API request failed with status {status}: {body}")]
    Api { status: u16, body: String },

    #[error("Response parsing failed: {0}")]
    Parse(String),

    #[error("Request body serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid header value: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),

    #[error("Endpoint not found: {0}")]
    NotFound(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("CSRF token missing")]
    MissingCsrfToken,

    #[error("URL parsing failed: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("WebSocket error: {0}")]
    WebSocket(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

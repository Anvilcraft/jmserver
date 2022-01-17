use hyper::StatusCode;
use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug)]
pub enum JMError {
    #[error("File read error: {0}")]
    Read(#[from] std::io::Error),
    #[error("Deserialize error: {0}")]
    Deserialize(#[from] toml::de::Error),
    #[error("Database connection error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Axum error: {0}")]
    Axum(#[from] hyper::Error),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("URL parse error: {0}")]
    Url(#[from] ParseError),
    #[error("Invalid response code: {0}")]
    InvalidResponse(StatusCode),
}

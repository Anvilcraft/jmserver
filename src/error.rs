use std::string::FromUtf8Error;

use axum::extract::{multipart::MultipartError, rejection::QueryRejection};
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

#[derive(Error, Debug)]
pub enum APIError {
    #[error("SQL error: {0}")]
    Sql(#[from] sqlx::Error),
    #[error("Multipart form error: {0}")]
    Multipart(#[from] MultipartError),
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    Forbidden(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Internal(String),
    #[error("JMService error: {0}")]
    Service(#[from] ServiceError),
    #[error("Query rejection: {0}")]
    Query(#[from] QueryRejection),
    #[error("Decode error: {0}")]
    Decode(#[from] FromUtf8Error),
}

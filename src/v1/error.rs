use std::convert::Infallible;

use axum::{
    body::{Bytes, Full},
    extract::multipart::MultipartError,
    response::IntoResponse,
    Json,
};
use reqwest::StatusCode;
use thiserror::Error;

use super::models::ErrorResponse;
use crate::ipfs::error::IPFSError;

#[derive(Error, Debug)]
pub enum APIError {
    #[error("SQL error: {0}")]
    Sql(#[from] sqlx::Error),
    #[error("Multipart form error: {0}")]
    Multipart(#[from] MultipartError),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("IPFS error: {0}")]
    IPFS(#[from] IPFSError),
}

impl ErrorResponse {
    fn new(status: StatusCode, message: Option<String>) -> Self {
        let reason = status.canonical_reason().unwrap_or_default();
        Self {
            status,
            error: message.unwrap_or(reason.to_string()),
        }
    }
}

impl IntoResponse for APIError {
    type Body = Full<Bytes>;

    type BodyError = Infallible;

    fn into_response(self) -> axum::http::Response<Self::Body> {
        let res = match self {
            APIError::Sql(err) => match err {
                sqlx::Error::RowNotFound => ErrorResponse::new(StatusCode::NOT_FOUND, None),
                _ => ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None),
            },
            APIError::Multipart(_) => ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None),
            APIError::BadRequest(err) => ErrorResponse::new(StatusCode::BAD_REQUEST, Some(err)),
            APIError::IPFS(_) => ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None),
        };
        let status = res.status.clone();
        (status, Json(res)).into_response()
    }
}

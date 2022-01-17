use std::convert::Infallible;

use axum::{
    body::{Bytes, Full},
    extract::{multipart::MultipartError, rejection::QueryRejection},
    response::IntoResponse,
    Json,
};
use reqwest::StatusCode;
use thiserror::Error;

use super::models::ErrorResponse;
use crate::error::ServiceError;

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
}

impl ErrorResponse {
    fn new(status: StatusCode, message: Option<String>) -> Self {
        let reason = status.canonical_reason().unwrap_or_default();
        Self {
            status,
            error: message.unwrap_or_else(|| reason.to_string()),
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
            APIError::Unauthorized(err) => ErrorResponse::new(StatusCode::UNAUTHORIZED, Some(err)),
            APIError::Forbidden(err) => ErrorResponse::new(StatusCode::FORBIDDEN, Some(err)),
            APIError::NotFound(err) => ErrorResponse::new(StatusCode::NOT_FOUND, Some(err)),
            APIError::Internal(err) => ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, Some(err)),
            APIError::Service(_) => ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None),
            APIError::Query(_) => ErrorResponse::new(StatusCode::BAD_REQUEST, None),
        };
        let status = res.status;
        (status, Json(res)).into_response()
    }
}

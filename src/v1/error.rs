use std::convert::Infallible;

use axum::{
    body::{Bytes, Full},
    response::IntoResponse,
    Json,
};
use reqwest::StatusCode;

use super::models::ErrorResponse;
use crate::error::{APIError, ServiceError};

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
            APIError::Multipart(_) => ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                Some("Invalid Multipart Form".to_string()),
            ),
            APIError::BadRequest(err) => ErrorResponse::new(StatusCode::BAD_REQUEST, Some(err)),
            APIError::Unauthorized(err) => ErrorResponse::new(StatusCode::UNAUTHORIZED, Some(err)),
            APIError::Forbidden(err) => ErrorResponse::new(StatusCode::FORBIDDEN, Some(err)),
            APIError::NotFound(err) => ErrorResponse::new(StatusCode::NOT_FOUND, Some(err)),
            APIError::Internal(err) => {
                ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, Some(err))
            }
            APIError::Service(err) => ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(err.get_response_message()),
            ),
            APIError::Query(_) => ErrorResponse::new(StatusCode::BAD_REQUEST, None),
            APIError::Decode(_) => ErrorResponse::new(StatusCode::BAD_REQUEST, None),
        };
        let status = res.status;
        (status, Json(res)).into_response()
    }
}

impl ServiceError {
    fn get_response_message(&self) -> String {
        match self {
            ServiceError::Reqwest(_) => "Reqwest error".to_string(),
            ServiceError::Url(_) => "URL parse error".to_string(),
            ServiceError::InvalidResponse(code) => format!("Invalid response code: {}", code),
        }
    }
}

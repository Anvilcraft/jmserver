use std::convert::Infallible;

use axum::{
    body::{Bytes, Full},
    response::IntoResponse,
    Json,
};
use reqwest::StatusCode;

use super::models::ErrorResponse;

impl ErrorResponse {
    fn new(status: StatusCode, message: Option<String>) -> Self {
        ErrorResponse {
            status,
            error: message,
        }
    }
}

impl IntoResponse for ErrorResponse {
    type Body = Full<Bytes>;

    type BodyError = Infallible;

    fn into_response(self) -> axum::http::Response<Self::Body> {
        let status = self.status.clone();
        (status, Json(self)).into_response()
    }
}

impl From<sqlx::Error> for ErrorResponse {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => {
                Self::new(StatusCode::NOT_FOUND, Some("Not found".to_string()))
            }
            _ => Self::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some("Internal Server Error".to_string()),
            ),
        }
    }
}

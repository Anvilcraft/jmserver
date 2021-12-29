use std::{convert::Infallible, string::FromUtf8Error};

use axum::{
    body::{Bytes, Empty},
    response::IntoResponse,
};
use reqwest::StatusCode;

pub struct Error(StatusCode);

impl Error {
    pub fn new() -> Self {
        Error(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl IntoResponse for Error {
    type Body = Empty<Bytes>;

    type BodyError = Infallible;

    fn into_response(self) -> axum::http::Response<Self::Body> {
        self.0.into_response()
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Error(StatusCode::NOT_FOUND),
            _ => Error(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error(err.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_: FromUtf8Error) -> Self {
        Error(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

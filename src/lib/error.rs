use std::{convert::Infallible, net::AddrParseError};

use axum::{
    body::{Bytes, Full},
    response::IntoResponse,
};
use hyper::{header::ToStrError, StatusCode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExtractError {
    #[error("`{0}` header is missing")]
    HeaderMissing(String),
    #[error("Header value contains illegal character: {0}")]
    ToStr(#[from] ToStrError),
    #[error("Header value is not a valid IP address: {0}")]
    IPParse(#[from] AddrParseError),
}

impl IntoResponse for ExtractError {
    type Body = Full<Bytes>;

    type BodyError = Infallible;

    fn into_response(self) -> hyper::Response<Self::Body> {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

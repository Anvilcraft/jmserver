use std::{convert::Infallible, string::FromUtf8Error};

use axum::{
    body::{Bytes, Empty},
    response::IntoResponse,
};
use reqwest::StatusCode;
use thiserror::Error;

use crate::ipfs::error::IPFSError;

#[derive(Error, Debug)]
pub enum CDNError {
    #[error("SQL error: {0}")]
    SQL(#[from] sqlx::Error),
    #[error("IPFS error: {0}")]
    IPFS(#[from] IPFSError),
    #[error("Decode error: {0}")]
    Decode(#[from] FromUtf8Error),
    #[error("Internal server error")]
    Internal,
}

impl IntoResponse for CDNError {
    type Body = Empty<Bytes>;

    type BodyError = Infallible;

    fn into_response(self) -> axum::http::Response<Self::Body> {
        let status = match self {
            CDNError::SQL(err) => match err {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        status.into_response()
    }
}

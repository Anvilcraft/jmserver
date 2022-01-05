use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug)]
pub enum IPFSError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("URL parse error: {0}")]
    URL(#[from] ParseError),
}

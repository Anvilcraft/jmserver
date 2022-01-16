use reqwest::Url;
use serde::Deserialize;
use std::{net::SocketAddr, sync::Arc};

use crate::{error::JMError, JMService, JMServiceInner};

#[derive(Deserialize)]
pub struct Config {
    pub addr: SocketAddr,
    pub database: String,
    pub cdn: String,
    pub ipfs_api: Url,
}

impl Config {
    pub fn service(&self) -> Result<JMService, JMError> {
        let client = reqwest::ClientBuilder::new().user_agent("curl").build()?;
        Ok(Arc::new(JMServiceInner {
            client,
            ipfs_url: self.ipfs_api.clone(),
            cdn_url: self.cdn.clone(),
        }))
    }
}

impl JMServiceInner {
    pub fn cdn_url(&self) -> String {
        self.cdn_url.clone()
    }
}

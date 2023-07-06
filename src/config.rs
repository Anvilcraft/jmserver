use reqwest::Url;
use serde::Deserialize;
use sqlx::PgPool;
use std::{net::SocketAddr, sync::Arc};

use crate::{error::JMError, JMService, JMServiceInner};

#[derive(Deserialize)]
pub struct Config {
    pub addr: SocketAddr,
    pub database: String,
    pub cdn: String,
    pub ipfs_api: Url,
    pub matrix_url: Url,
    pub matrix_token: String,
    pub matrix_domain: String,
}

impl Config {
    pub fn service(&self, db_pool: PgPool) -> Result<JMService, JMError> {
        let client = reqwest::ClientBuilder::new().user_agent("curl").build()?;
        Ok(Arc::new(JMServiceInner {
            client,
            db_pool,
            ipfs_url: self.ipfs_api.clone(),
            cdn_url: self.cdn.clone(),
            matrix_url: self.matrix_url.clone(),
            matrix_token: self.matrix_token.clone(),
            matrix_domain: self.matrix_domain.clone(),
        }))
    }
}

impl JMServiceInner {
    pub fn cdn_url(&self) -> String {
        self.cdn_url.clone()
    }
}

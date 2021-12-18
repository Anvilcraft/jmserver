use std::net::SocketAddr;
use reqwest::Url;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub addr: SocketAddr,
    pub database: String,
    pub cdn: String,
    pub ipfs_api: Url,
}

pub struct ConfVars {
    pub cdn: String,
    pub ipfs_api: Url,
}

impl Config {

    pub fn vars(&self) -> ConfVars {
        ConfVars {
            cdn: self.cdn.clone(),
            ipfs_api: self.ipfs_api.clone(),
        }
    }

}

impl Clone for ConfVars {
    fn clone(&self) -> Self {
        Self { cdn: self.cdn.clone(), ipfs_api: self.ipfs_api.clone() }
    }
}
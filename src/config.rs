use std::net::SocketAddr;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub addr: SocketAddr,
    pub database: String,
    pub cdn: String,
}

pub struct ConfVars {
    pub cdn: String,
}

impl Config {

    pub fn vars(&self) -> ConfVars {
        ConfVars {
            cdn: self.cdn.clone(),
        }
    }

}

impl Clone for ConfVars {
    fn clone(&self) -> Self {
        Self { cdn: self.cdn.clone() }
    }
}
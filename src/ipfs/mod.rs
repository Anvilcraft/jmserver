use std::time::Duration;

use axum::{body::Bytes, http::request};
use reqwest::{
    multipart::{Form, Part},
    Body, Client, Response, Url,
};
use serde::{Deserialize, Serialize};

use crate::config::ConfVars;

use self::error::IPFSError;

pub(crate) mod error;

#[derive(Deserialize)]
pub struct IPFSFile {
    #[serde(rename = "Hash")]
    pub hash: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Size")]
    pub size: String,
}

#[derive(Serialize)]
pub struct CatQuery {
    pub arg: String,
}

#[derive(Serialize)]
pub struct AddQuery {
    pub pin: bool,
}

#[derive(Serialize)]
pub struct PinQuery {
    pub arg: String,
}

pub struct IpfsClient {
    url: Url,
    client: Client,
}

impl IpfsClient {
    pub async fn cat(&self, cid: String) -> Result<Response, IPFSError> {
        let request = self
            .client
            .post(self.url.join("/api/v0/cat")?)
            .query(&CatQuery::new(cid));
        Ok(request.send().await?)
    }

    pub async fn add(&self, file: Bytes, filename: String) -> Result<IPFSFile, IPFSError> {
        let request = self
            .client
            .post(self.url.join("/api/v0/add")?)
            .query(&AddQuery::new(false))
            .multipart(Form::new().part("file", Part::stream(file).file_name(filename)));
        let response = request.send().await?;
        let res: IPFSFile = response.json().await?;
        Ok(res)
    }

    pub async fn pin(&self, cid: String) -> Result<(), IPFSError> {
        let request = self
            .client
            .post(self.url.join("/api/v0/pin/add")?)
            .query(&PinQuery::new(cid))
            .timeout(Duration::from_secs(60));
        let response = request.send().await?;
        Ok(())
    }
}

impl CatQuery {
    pub fn new(cid: String) -> Self {
        Self { arg: cid }
    }
}

impl AddQuery {
    pub fn new(pin: bool) -> Self {
        Self { pin }
    }
}

impl PinQuery {
    pub fn new(cid: String) -> Self {
        Self { arg: cid }
    }
}

impl ConfVars {
    pub fn ipfs_client(&self) -> Result<IpfsClient, IPFSError> {
        let client = reqwest::ClientBuilder::new().user_agent("curl").build()?;
        Ok(IpfsClient {
            url: self.ipfs_api.clone(),
            client,
        })
    }
}

use reqwest::{Client, Response, Result, Url};
use serde::{Deserialize, Serialize};

use crate::config::ConfVars;

#[derive(Deserialize)]
pub struct AddResponse {
    #[serde(rename = "Bytes")]
    pub bytes: String,
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

pub struct IpfsClient {
    url: Url,
    client: Client,
}

impl IpfsClient {

    pub fn cat_url(&self) -> Url {
        self.url.join("/api/v0/cat").expect("Something went wrong with the IPFS URL")
    }

    pub fn add_url(&self) -> Url {
        self.url.join("/api/v0/add").expect("Something went wrong with the IPFS URL")
    }

    pub async fn cat(&self, cid: String) -> Result<Response> {
        let request = self.client.post(self.cat_url()).query(&CatQuery::new(cid));
        request.send().await
    }
    
}

impl CatQuery {
    
    pub fn new(cid: String) -> Self {
        Self {
            arg: cid,
        }
    }

}

impl ConfVars {

    pub fn ipfs_client(&self) -> Result<IpfsClient> {
        let client =reqwest::ClientBuilder::new().user_agent("curl").build()?;
        Ok(IpfsClient {
            url: self.ipfs_api.clone(),
            client,
        })
    }
    
}
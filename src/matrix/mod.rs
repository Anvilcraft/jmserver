use serde::{Deserialize, Serialize};

use crate::{error::ServiceError, JMServiceInner};

#[derive(Serialize)]
pub struct Meme {
    pub category: String,
    pub filename: String,
    pub cid: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserID {
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct RoomID {
    pub room_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct EventID {
    pub event_id: String,
}

#[derive(Serialize)]
pub struct RegisterRequest {
    #[serde(rename = "type")]
    pub reg_type: String,
    pub username: String,
}

impl JMServiceInner {
    pub async fn add_meme(
        &self,
        category: String,
        filename: String,
        cid: String,
        user: String,
        id: i64,
    ) -> Result<(), ServiceError> {
        let meme = Meme {
            category,
            filename,
            cid,
        };
        let txid = meme.calc_txid(user.clone());
        let usr = self.check_user(user).await?;
        let room_id = self.join_room(&usr).await?;
        let path = format!(
            "/_matrix/client/r0/rooms/{}/send/es.jensmem.meme/{}",
            &room_id, txid
        );
        let url = self.matrix_url.join(path.as_str())?;
        let req = self
            .client
            .put(url)
            .bearer_auth(self.matrix_token.clone())
            .query(&usr)
            .json(&meme);
        let res = req.send().await?;
        if res.status().is_success() {
            let event: EventID = res.json().await?;
            let path = format!(
                "/_matrix/client/r0/rooms/{}/state/es.jensmem.index/{}",
                &room_id, id
            );
            let req = self
                .client
                .put(self.matrix_url.join(path.as_str())?)
                .bearer_auth(self.matrix_token.clone())
                .json(&event);
            let res = req.send().await?;
            if res.status().is_success() {
                Ok(())
            } else {
                Err(ServiceError::InvalidResponse(res.status()))
            }
        } else {
            Err(ServiceError::InvalidResponse(res.status()))
        }
    }

    async fn check_user(&self, user: String) -> Result<UserID, ServiceError> {
        let username = format!("jm_{}", user);
        let user = self.get_mxid(username.clone());
        let req = self
            .client
            .get(self.matrix_url.join("/_matrix/client/r0/account/whoami")?)
            .bearer_auth(self.matrix_token.clone())
            .query(&user);
        let res = req.send().await?;
        if res.status().is_success() {
            let mxid: UserID = res.json().await?;
            Ok(mxid)
        } else {
            let mxid = self.register_user(username).await?;
            Ok(mxid)
        }
    }

    async fn register_user(&self, username: String) -> Result<UserID, ServiceError> {
        let req = self
            .client
            .post(self.matrix_url.join("/_matrix/client/r0/register")?)
            .bearer_auth(self.matrix_token.clone())
            .json(&RegisterRequest::new(username));
        let res = req.send().await?;
        if res.status().is_success() {
            let user: UserID = res.json().await?;
            Ok(user)
        } else {
            Err(ServiceError::InvalidResponse(res.status()))
        }
    }

    async fn join_room(&self, user: &UserID) -> Result<String, ServiceError> {
        let req = self
            .client
            .post(
                self.matrix_url
                    .join("/_matrix/client/r0/join/%23memes%3Atilera.org")?,
            )
            .bearer_auth(self.matrix_token.clone())
            .query(user);

        let res = req.send().await?;
        if res.status().is_success() {
            let room: RoomID = res.json().await?;
            Ok(room.room_id)
        } else {
            Err(ServiceError::InvalidResponse(res.status()))
        }
    }

    fn get_mxid(&self, username: String) -> UserID {
        UserID {
            user_id: format!("@{}:{}", username, self.matrix_domain.clone()),
        }
    }
}

impl RegisterRequest {
    pub fn new(username: String) -> Self {
        Self {
            reg_type: "m.login.application_service".to_string(),
            username,
        }
    }
}

impl Meme {
    pub fn calc_txid(&self, user: String) -> String {
        let txid = format!("{}/{}/{}/{}", user, self.category, self.filename, self.cid);
        urlencoding::encode(txid.as_str()).into_owned()
    }
}

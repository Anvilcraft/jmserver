use reqwest::StatusCode;
use serde::{Deserialize, Serialize, Serializer};

use crate::models::{Category, Meme, User, UserIdentifier};

fn serialize_status<S>(x: &StatusCode, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u16(x.as_u16())
}

#[derive(Serialize)]
pub struct V1Meme {
    pub id: String,
    pub link: String,
    pub category: String,
    pub user: String,
    pub timestamp: String,
    pub ipfs: String,
}

//Responses

#[derive(Serialize)]
pub struct MemesResponse {
    pub status: i32,
    pub error: Option<String>,
    pub memes: Option<Vec<V1Meme>>,
}

#[derive(Serialize)]
pub struct MemeResponse {
    pub status: i32,
    pub error: Option<String>,
    pub meme: Option<V1Meme>,
}

#[derive(Serialize)]
pub struct CategoriesResponse {
    pub status: i32,
    pub error: Option<String>,
    pub categories: Option<Vec<Category>>,
}

#[derive(Serialize)]
pub struct CategoryResponse {
    pub status: i32,
    pub error: Option<String>,
    pub category: Option<Category>,
}

#[derive(Serialize)]
pub struct UsersResponse {
    pub status: i32,
    pub error: Option<String>,
    pub users: Option<Vec<User>>,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub status: i32,
    pub error: Option<String>,
    pub user: Option<User>,
}

#[derive(Serialize)]
pub struct UploadResponse {
    pub status: i32,
    pub error: Option<String>,
    pub files: Option<Vec<String>>,
    pub token: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    #[serde(serialize_with = "serialize_status")]
    pub status: StatusCode,
    pub error: String,
}

//Query

#[derive(Deserialize)]
pub struct IDQuery {
    pub id: String,
}

#[derive(Deserialize)]
pub struct MemeIDQuery {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct UserIDQuery {
    pub id: Option<String>,
    pub token: Option<String>,
    pub name: Option<String>,
}

impl V1Meme {
    pub fn new(meme: Meme, cdn: String) -> Self {
        Self {
            id: meme.id.to_string(),
            link: format!("{}/{}/{}", cdn, meme.userid, meme.filename),
            category: meme.category,
            user: meme.username,
            timestamp: meme.timestamp.to_string(),
            ipfs: meme.ipfs,
        }
    }
}

impl From<UserIDQuery> for UserIdentifier {
    fn from(query: UserIDQuery) -> Self {
        if let Some(id) = query.id {
            Self::Id(id)
        } else if let Some(token) = query.token {
            Self::Token(token)
        } else if let Some(name) = query.name {
            Self::Username(name)
        } else {
            Self::Null
        }
    }
}

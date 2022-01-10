use reqwest::StatusCode;
use serde::{Deserialize, Serialize, Serializer};

fn serialize_status<S>(x: &StatusCode, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u16(x.as_u16())
}

#[derive(Serialize)]
pub struct Meme {
    pub id: String,
    pub link: String,
    pub category: String,
    pub user: String,
    pub timestamp: String,
    pub ipfs: String,
}

#[derive(Serialize)]
pub struct Category {
    pub id: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub userdir: String,
    pub tokenhash: String,
    pub dayuploads: i32,
}

//Responses

#[derive(Serialize)]
pub struct MemesResponse {
    pub status: i32,
    pub error: Option<String>,
    pub memes: Option<Vec<Meme>>,
}

#[derive(Serialize)]
pub struct MemeResponse {
    pub status: i32,
    pub error: Option<String>,
    pub meme: Option<Meme>,
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

#[derive(Deserialize)]
pub struct MemeFilterQuery {
    pub category: Option<String>,
    pub user: Option<String>,
    pub search: Option<String>,
}

use serde::Serialize;

#[derive(Serialize)]
pub struct Meme {
    pub id: i32,
    pub filename: String,
    pub userid: String,
    pub username: String,
    pub category: String,
    pub timestamp: i64,
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

pub enum UserIdentifier {
    Id(String),
    Token(String),
    Username(String),
    Null,
}

pub struct MemeOptions {
    pub category: Option<String>,
    pub user_id: Option<String>,
    pub username: Option<String>,
    pub search: Option<String>,
    pub limit: Option<i32>,
    pub after: Option<i32>,
}

impl MemeOptions {
    pub fn empty() -> Self {
        Self {
            category: None,
            user_id: None,
            username: None,
            search: None,
            limit: None,
            after: None,
        }
    }
}

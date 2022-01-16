use serde::{Deserialize, Serialize};

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

#[derive(Deserialize)]
pub struct MemeFilter {
    pub category: Option<String>,
    pub user: Option<String>,
    pub search: Option<String>,
}

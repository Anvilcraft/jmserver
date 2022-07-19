use crate::models::{Meme, User};
use serde::Serialize;

#[derive(Serialize)]
pub struct V2Meme {
    pub id: i32,
    pub filename: String,
    pub ipfs: String,
    pub category: String,
    pub user: String,
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct V2User {
    pub id: String,
    pub name: String,
    pub dayuploads: i32,
}

#[derive(Serialize)]
pub struct CDNEntry {
    pub directories: Vec<String>,
    pub files: Vec<String>,
}

#[derive(Serialize)]
pub struct CDNFile {
    pub cid: String,
    pub filename: String,
}

impl From<Meme> for V2Meme {
    fn from(meme: Meme) -> Self {
        Self {
            id: meme.id,
            filename: meme.filename,
            category: meme.category,
            user: meme.userid,
            timestamp: meme.timestamp,
            ipfs: meme.ipfs,
        }
    }
}

impl From<User> for V2User {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            dayuploads: user.dayuploads,
        }
    }
}

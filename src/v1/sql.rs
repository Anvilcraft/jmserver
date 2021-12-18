use crate::v1::models::{Meme, MemeFilterQuery, Category, User, UserIDQuery};
use sqlx::{MySqlPool, Result, Row};
use sqlx::mysql::MySqlRow;

pub struct DBMeme {
    pub id: i32,
    pub filename: String,
    pub user: String,
    pub userdir: String,
    pub category: String,
    pub timestamp: i64,
    pub ipfs: Option<String>,
}

impl Meme {

    pub fn new(meme: DBMeme, cdn: String) -> Self {
        Self {
            id: meme.id.to_string(),
            link: format!("{}/{}/{}", cdn, meme.userdir, meme.filename),
            category: meme.category,
            user: meme.user,
            timestamp: meme.timestamp.to_string(),
            ipfs: meme.ipfs,
        }
    }

    pub async fn get(id: i32, pool: &MySqlPool, cdn: String) -> Result<Meme> {
        let q: Meme = sqlx::query("SELECT memes.id, user, filename, category, name, UNIX_TIMESTAMP(timestamp) AS ts, cid FROM memes, users WHERE memes.user = users.id AND memes.id=?").bind(id)
            .map(|row: MySqlRow| Meme::new(DBMeme {
                id: row.get("id"),
                filename: row.get("filename"),
                user: row.get("name"),
                userdir: row.get("user"),
                category: row.get("category"),
                timestamp: row.get("ts"),
                ipfs: row.get("cid"),
            }, cdn.clone()))
            .fetch_one(pool).await?;
        Ok(q)
    }

    pub async fn get_all(params: MemeFilterQuery, pool: &MySqlPool, cdn: String) -> Result<Vec<Meme>> {
        let q: Vec<Meme> = sqlx::query("SELECT memes.id, user, filename, category, name, UNIX_TIMESTAMP(timestamp) AS ts, cid FROM memes, users WHERE memes.user = users.id AND (category LIKE ? AND name LIKE ? AND filename LIKE ?) ORDER BY memes.id")
            .bind(params.category.unwrap_or(String::from("%")))
            .bind(format!("%{}%", params.user.unwrap_or(String::from(""))))
            .bind(format!("%{}%", params.search.unwrap_or(String::from(""))))
            .map(|row: MySqlRow| Meme::new(DBMeme {
                id: row.get("id"),
                filename: row.get("filename"),
                user: row.get("name"),
                userdir: row.get("user"),
                category: row.get("category"),
                timestamp: row.get("ts"),
                ipfs: row.get("cid"),
            }, cdn.clone()))
            .fetch_all(pool).await?;
        Ok(q)
    }

    pub async fn get_random(params: MemeFilterQuery, pool: &MySqlPool, cdn: String) -> Result<Meme> {
        let q: Meme = sqlx::query("SELECT memes.id, user, filename, category, name, UNIX_TIMESTAMP(timestamp) AS ts, cid FROM memes, users WHERE memes.user = users.id AND (category LIKE ? AND name LIKE ? AND filename LIKE ?) ORDER BY RAND() LIMIT 1")
            .bind(params.category.unwrap_or(String::from("%")))
            .bind(format!("%{}%", params.user.unwrap_or(String::from(""))))
            .bind(format!("%{}%", params.search.unwrap_or(String::from(""))))
            .map(|row: MySqlRow| Meme::new(DBMeme {
                id: row.get("id"),
                filename: row.get("filename"),
                user: row.get("name"),
                userdir: row.get("user"),
                category: row.get("category"),
                timestamp: row.get("ts"),
                ipfs: row.get("cid"),
            }, cdn.clone()))
            .fetch_one(pool).await?;
        Ok(q)
    }

}

impl Category {
    pub async fn get(id: &String, pool: &MySqlPool) -> Result<Category> {
        let q: Category = sqlx::query("SELECT * FROM categories WHERE id=?").bind(id)
            .map(|row: MySqlRow| Category {
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_one(pool).await?;
        Ok(q)
    }

    pub async fn get_all(pool: &MySqlPool) -> Result<Vec<Category>> {
        let q: Vec<Category> = sqlx::query("SELECT * FROM categories ORDER BY num")
            .map(|row: MySqlRow| Category {
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_all(pool).await?;
        Ok(q)
    }
}

impl User {

    pub async fn get(params: UserIDQuery, pool: &MySqlPool) -> Result<User> {
        let q: User = sqlx::query("SELECT id, name, MD5(token) AS hash, uploads FROM (SELECT id, name, IFNULL(count.uploads, 0) AS uploads FROM users LEFT JOIN (SELECT user, COUNT(*) AS uploads FROM memes WHERE DATE(timestamp) = CURDATE() GROUP BY (user)) AS count ON users.id = count.user) AS users, token WHERE users.id = token.uid AND (users.id LIKE ? OR token LIKE ? OR name LIKE ?) UNION SELECT id, name, 0 AS hash, 0 AS uploads FROM users WHERE id = '000'")
            .bind(params.id.unwrap_or(String::from("")))
            .bind(params.token.unwrap_or(String::from("")))
            .bind(params.name.unwrap_or(String::from("")))
            .map(|row: MySqlRow| User {
                id: row.get("id"),
                name: row.get("name"),
                userdir: row.get("id"),
                tokenhash: row.get("hash"),
                dayuploads: row.get("uploads"),
            })
            .fetch_one(pool).await?;
        Ok(q)
    }
    
    pub async fn get_all(pool: &MySqlPool) -> Result<Vec<User>> {
        let q: Vec<User> = sqlx::query("SELECT id, name, MD5(token) AS hash, uploads FROM (SELECT id, name, IFNULL(count.uploads, 0) AS uploads FROM users LEFT JOIN (SELECT user, COUNT(*) AS uploads FROM memes WHERE DATE(timestamp) = CURDATE() GROUP BY (user)) AS count ON users.id = count.user) AS users, token WHERE users.id = token.uid UNION SELECT id, name, 0 AS hash, 0 AS uploads FROM users WHERE id = '000'")
            .map(|row: MySqlRow| User {
                id: row.get("id"),
                name: row.get("name"),
                userdir: row.get("id"),
                tokenhash: row.get("hash"),
                dayuploads: row.get("uploads"),
            })
            .fetch_all(pool).await?;
        Ok(q)
    }
    
}

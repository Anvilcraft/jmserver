use crate::ipfs::IPFSFile;
use crate::models::{Category, Meme, MemeFilter, User, UserIdentifier};
use sqlx::mysql::MySqlRow;
use sqlx::{MySqlPool, Result, Row};

impl Meme {
    pub async fn get(id: i32, pool: &MySqlPool) -> Result<Option<Self>> {
        let q: Option<Self> = sqlx::query("SELECT memes.id, user, filename, category, name, UNIX_TIMESTAMP(timestamp) AS ts, cid FROM memes, users WHERE memes.user = users.id AND memes.id=?").bind(id)
            .map(|row: MySqlRow| Self {
                id: row.get("id"),
                filename: row.get("filename"),
                username: row.get("name"),
                userid: row.get("user"),
                category: row.get("category"),
                timestamp: row.get("ts"),
                ipfs: row.get("cid"),
            })
            .fetch_optional(pool).await?;
        Ok(q)
    }

    pub async fn get_all(filter: MemeFilter, pool: &MySqlPool) -> Result<Vec<Self>> {
        let q: Vec<Self> = sqlx::query("SELECT memes.id, user, filename, category, name, UNIX_TIMESTAMP(timestamp) AS ts, cid FROM memes, users WHERE memes.user = users.id AND (category LIKE ? AND name LIKE ? AND filename LIKE ?) ORDER BY memes.id")
            .bind(filter.category.unwrap_or_else(|| String::from("%")))
            .bind(format!("%{}%", filter.user.unwrap_or_else(String::new)))
            .bind(format!("%{}%", filter.search.unwrap_or_else(String::new)))
            .map(|row: MySqlRow| Self {
                id: row.get("id"),
                filename: row.get("filename"),
                username: row.get("name"),
                userid: row.get("user"),
                category: row.get("category"),
                timestamp: row.get("ts"),
                ipfs: row.get("cid"),
            })
            .fetch_all(pool).await?;
        Ok(q)
    }

    pub async fn get_random(filter: MemeFilter, pool: &MySqlPool) -> Result<Self> {
        let q: Self = sqlx::query("SELECT memes.id, user, filename, category, name, UNIX_TIMESTAMP(timestamp) AS ts, cid FROM memes, users WHERE memes.user = users.id AND (category LIKE ? AND name LIKE ? AND filename LIKE ?) ORDER BY RAND() LIMIT 1")
            .bind(filter.category.unwrap_or_else(|| String::from("%")))
            .bind(format!("%{}%", filter.user.unwrap_or_else(String::new)))
            .bind(format!("%{}%", filter.search.unwrap_or_else(String::new)))
            .map(|row: MySqlRow| Self {
                id: row.get("id"),
                filename: row.get("filename"),
                username: row.get("name"),
                userid: row.get("user"),
                category: row.get("category"),
                timestamp: row.get("ts"),
                ipfs: row.get("cid"),
            })
            .fetch_one(pool).await?;
        Ok(q)
    }
}

impl Category {
    pub async fn get(id: &String, pool: &MySqlPool) -> Result<Option<Self>> {
        let q: Option<Self> = sqlx::query("SELECT * FROM categories WHERE id=?")
            .bind(id)
            .map(|row: MySqlRow| Self {
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_optional(pool)
            .await?;
        Ok(q)
    }

    pub async fn get_all(pool: &MySqlPool) -> Result<Vec<Self>> {
        let q: Vec<Self> = sqlx::query("SELECT * FROM categories ORDER BY num")
            .map(|row: MySqlRow| Self {
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_all(pool)
            .await?;
        Ok(q)
    }

    pub async fn add_meme(
        &self,
        user: &User,
        file: &IPFSFile,
        ip: &String,
        pool: &MySqlPool,
    ) -> Result<u64> {
        let mut tx = pool.begin().await?;
        sqlx::query("INSERT INTO memes (filename, user, category, timestamp, ip, cid) VALUES (?, ?, ?, NOW(), ?, ?)")
        .bind(&file.name)
        .bind(&user.id)
        .bind(&self.id)
        .bind(ip)
        .bind(&file.hash)
        .execute(&mut tx).await?;
        let id: u64 = sqlx::query("SELECT LAST_INSERT_ID() as id")
            .map(|row: MySqlRow| row.get("id"))
            .fetch_one(&mut tx)
            .await?;
        tx.commit().await?;
        Ok(id)
    }
}

impl User {
    pub async fn get(identifier: UserIdentifier, pool: &MySqlPool) -> Result<Option<Self>> {
        let query = match identifier {
            UserIdentifier::Id(id) => sqlx::query("SELECT id, name, IFNULL(MD5(token), '0') AS hash, uploads FROM (SELECT id, name, IFNULL(count.uploads, 0) AS uploads FROM users LEFT JOIN (SELECT user, COUNT(*) AS uploads FROM memes WHERE DATE(timestamp) = CURDATE() GROUP BY (user)) AS count ON users.id = count.user) AS users LEFT JOIN token ON users.id = token.uid WHERE users.id = ?").bind(id),
            UserIdentifier::Token(token) => sqlx::query("SELECT id, name, IFNULL(MD5(token), '0') AS hash, uploads FROM (SELECT id, name, IFNULL(count.uploads, 0) AS uploads FROM users LEFT JOIN (SELECT user, COUNT(*) AS uploads FROM memes WHERE DATE(timestamp) = CURDATE() GROUP BY (user)) AS count ON users.id = count.user) AS users LEFT JOIN token ON users.id = token.uid WHERE token = ?").bind(token),
            UserIdentifier::Username(name) => sqlx::query("SELECT id, name, IFNULL(MD5(token), '0') AS hash, uploads FROM (SELECT id, name, IFNULL(count.uploads, 0) AS uploads FROM users LEFT JOIN (SELECT user, COUNT(*) AS uploads FROM memes WHERE DATE(timestamp) = CURDATE() GROUP BY (user)) AS count ON users.id = count.user) AS users LEFT JOIN token ON users.id = token.uid WHERE name = ?").bind(name),
            UserIdentifier::Null => sqlx::query("SELECT id, name, '0' AS hash, 0 AS uploads FROM users WHERE id = '000'"),
        };
        let q: Option<Self> = query
            .map(|row: MySqlRow| Self {
                id: row.get("id"),
                name: row.get("name"),
                userdir: row.get("id"),
                tokenhash: row.get("hash"),
                dayuploads: row.get("uploads"),
            })
            .fetch_optional(pool)
            .await?;
        Ok(q)
    }

    pub async fn get_all(pool: &MySqlPool) -> Result<Vec<Self>> {
        let q: Vec<Self> = sqlx::query("SELECT id, name, IFNULL(MD5(token), '0') AS hash, uploads FROM (SELECT id, name, IFNULL(count.uploads, 0) AS uploads FROM users LEFT JOIN (SELECT user, COUNT(*) AS uploads FROM memes WHERE DATE(timestamp) = CURDATE() GROUP BY (user)) AS count ON users.id = count.user) AS users LEFT JOIN token ON users.id = token.uid")
            .map(|row: MySqlRow| Self {
                id: row.get("id"),
                name: row.get("name"),
                userdir: row.get("id"),
                tokenhash: row.get("hash"),
                dayuploads: row.get("uploads"),
            })
            .fetch_all(pool).await?;
        Ok(q)
    }

    pub async fn check_token(token: &String, pool: &MySqlPool) -> Result<Option<Self>> {
        let user = Self::get(UserIdentifier::Token(token.clone()), pool).await?;
        Ok(user)
    }
}

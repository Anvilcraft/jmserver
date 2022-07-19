use crate::ipfs::IPFSFile;
use crate::models::{Category, Meme, MemeOptions, User, UserIdentifier};
use crate::JMServiceInner;
use sqlx::mysql::MySqlRow;
use sqlx::{MySqlPool, Result, Row};

impl Category {
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

impl JMServiceInner {
    pub async fn get_meme(&self, id: i32) -> Result<Option<Meme>> {
        let q: Option<Meme> = sqlx::query("SELECT memes.id, user, filename, category, name, UNIX_TIMESTAMP(timestamp) AS ts, cid FROM memes, users WHERE memes.user = users.id AND memes.id=?").bind(id)
            .map(|row: MySqlRow| Meme {
                id: row.get("id"),
                filename: row.get("filename"),
                username: row.get("name"),
                userid: row.get("user"),
                category: row.get("category"),
                timestamp: row.get("ts"),
                ipfs: row.get("cid"),
            })
            .fetch_optional(&self.db_pool).await?;
        Ok(q)
    }

    pub async fn get_memes(&self, filter: MemeOptions) -> Result<Vec<Meme>> {
        let q: Vec<Meme> = sqlx::query("SELECT memes.id, user, filename, category, name, UNIX_TIMESTAMP(timestamp) AS ts, cid FROM memes, users WHERE memes.user = users.id AND (category LIKE ? AND name LIKE ? AND filename LIKE ? AND memes.user LIKE ? AND memes.id > ?) ORDER BY memes.id")
            .bind(filter.category.unwrap_or_else(|| String::from("%")))
            .bind(format!("%{}%", filter.username.unwrap_or_else(String::new)))
            .bind(format!("%{}%", filter.search.unwrap_or_else(String::new)))
            .bind(filter.user_id.unwrap_or_else(|| String::from("%")))
            .bind(filter.after.unwrap_or(0))
            .map(|row: MySqlRow| Meme {
                id: row.get("id"),
                filename: row.get("filename"),
                username: row.get("name"),
                userid: row.get("user"),
                category: row.get("category"),
                timestamp: row.get("ts"),
                ipfs: row.get("cid"),
            })
            .fetch_all(&self.db_pool).await?;
        Ok(q)
    }

    pub async fn get_random_meme(&self, filter: MemeOptions) -> Result<Meme> {
        let q: Meme = sqlx::query("SELECT memes.id, user, filename, category, name, UNIX_TIMESTAMP(timestamp) AS ts, cid FROM memes, users WHERE memes.user = users.id AND (category LIKE ? AND name LIKE ? AND filename LIKE ? AND memes.user LIKE ? AND memes.id > ?) ORDER BY RAND() LIMIT 1")
            .bind(filter.category.unwrap_or_else(|| String::from("%")))
            .bind(format!("%{}%", filter.username.unwrap_or_else(String::new)))
            .bind(format!("%{}%", filter.search.unwrap_or_else(String::new)))
            .bind(filter.user_id.unwrap_or_else(|| String::from("%")))
            .bind(filter.after.unwrap_or(0))
            .map(|row: MySqlRow| Meme {
                id: row.get("id"),
                filename: row.get("filename"),
                username: row.get("name"),
                userid: row.get("user"),
                category: row.get("category"),
                timestamp: row.get("ts"),
                ipfs: row.get("cid"),
            })
            .fetch_one(&self.db_pool).await?;
        Ok(q)
    }

    pub async fn get_user_meme(&self, user_id: String, filename: String) -> Result<Option<Meme>> {
        let q: Option<Meme> = sqlx::query("SELECT memes.id, user, filename, category, name, UNIX_TIMESTAMP(timestamp) AS ts, cid FROM memes, users WHERE memes.user = users.id AND memes.user = ? AND filename = ? ORDER BY memes.id DESC")
            .bind(user_id)
            .bind(filename)
            .map(|row: MySqlRow| Meme {
                id: row.get("id"),
                filename: row.get("filename"),
                username: row.get("name"),
                userid: row.get("user"),
                category: row.get("category"),
                timestamp: row.get("ts"),
                ipfs: row.get("cid"),
            })
            .fetch_optional(&self.db_pool).await?;
        Ok(q)
    }

    pub async fn get_category(&self, id: &String) -> Result<Option<Category>> {
        let q: Option<Category> = sqlx::query("SELECT * FROM categories WHERE id=?")
            .bind(id)
            .map(|row: MySqlRow| Category {
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_optional(&self.db_pool)
            .await?;
        Ok(q)
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>> {
        let q: Vec<Category> = sqlx::query("SELECT * FROM categories ORDER BY num")
            .map(|row: MySqlRow| Category {
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_all(&self.db_pool)
            .await?;
        Ok(q)
    }

    pub async fn get_user(&self, identifier: UserIdentifier) -> Result<Option<User>> {
        let query = match identifier {
            UserIdentifier::Id(id) => sqlx::query("SELECT id, name, IFNULL(MD5(token), '0') AS hash, uploads FROM (SELECT id, name, IFNULL(count.uploads, 0) AS uploads FROM users LEFT JOIN (SELECT user, COUNT(*) AS uploads FROM memes WHERE DATE(timestamp) = CURDATE() GROUP BY (user)) AS count ON users.id = count.user) AS users LEFT JOIN token ON users.id = token.uid WHERE users.id = ?").bind(id),
            UserIdentifier::Token(token) => sqlx::query("SELECT id, name, IFNULL(MD5(token), '0') AS hash, uploads FROM (SELECT id, name, IFNULL(count.uploads, 0) AS uploads FROM users LEFT JOIN (SELECT user, COUNT(*) AS uploads FROM memes WHERE DATE(timestamp) = CURDATE() GROUP BY (user)) AS count ON users.id = count.user) AS users LEFT JOIN token ON users.id = token.uid WHERE token = ?").bind(token),
            UserIdentifier::Username(name) => sqlx::query("SELECT id, name, IFNULL(MD5(token), '0') AS hash, uploads FROM (SELECT id, name, IFNULL(count.uploads, 0) AS uploads FROM users LEFT JOIN (SELECT user, COUNT(*) AS uploads FROM memes WHERE DATE(timestamp) = CURDATE() GROUP BY (user)) AS count ON users.id = count.user) AS users LEFT JOIN token ON users.id = token.uid WHERE name = ?").bind(name),
            UserIdentifier::Null => sqlx::query("SELECT id, name, '0' AS hash, 0 AS uploads FROM users WHERE id = '000'"),
        };
        let q: Option<User> = query
            .map(|row: MySqlRow| User {
                id: row.get("id"),
                name: row.get("name"),
                userdir: row.get("id"),
                tokenhash: row.get("hash"),
                dayuploads: row.get("uploads"),
            })
            .fetch_optional(&self.db_pool)
            .await?;
        Ok(q)
    }

    pub async fn get_users(&self) -> Result<Vec<User>> {
        let q: Vec<User> = sqlx::query("SELECT id, name, IFNULL(MD5(token), '0') AS hash, uploads FROM (SELECT id, name, IFNULL(count.uploads, 0) AS uploads FROM users LEFT JOIN (SELECT user, COUNT(*) AS uploads FROM memes WHERE DATE(timestamp) = CURDATE() GROUP BY (user)) AS count ON users.id = count.user) AS users LEFT JOIN token ON users.id = token.uid")
            .map(|row: MySqlRow| User {
                id: row.get("id"),
                name: row.get("name"),
                userdir: row.get("id"),
                tokenhash: row.get("hash"),
                dayuploads: row.get("uploads"),
            })
            .fetch_all(&self.db_pool).await?;
        Ok(q)
    }

    pub async fn check_token(&self, token: &String) -> Result<Option<User>> {
        let user = self.get_user(UserIdentifier::Token(token.clone())).await?;
        Ok(user)
    }

    pub async fn add_meme_sql(
        &self,
        user: &User,
        file: &IPFSFile,
        ip: &String,
        category: &Category,
    ) -> Result<u64> {
        let mut tx = self.db_pool.begin().await?;
        sqlx::query("INSERT INTO memes (filename, user, category, timestamp, ip, cid) VALUES (?, ?, ?, NOW(), ?, ?)")
        .bind(&file.name)
        .bind(&user.id)
        .bind(&category.id)
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

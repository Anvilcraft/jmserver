use sqlx::{mysql::MySqlRow, MySqlPool, Result, Row};

pub async fn get_cid(user: String, filename: String, pool: &MySqlPool) -> Result<String> {
    let q: String =
        sqlx::query("SELECT cid FROM memes WHERE user = ? AND filename = ? ORDER BY id DESC")
            .bind(user)
            .bind(filename)
            .map(|row: MySqlRow| row.get("cid"))
            .fetch_one(pool)
            .await?;
    Ok(q)
}

pub async fn get_memes(user: String, pool: &MySqlPool) -> Result<Vec<String>> {
    let q: Vec<String> = sqlx::query("SELECT filename FROM memes WHERE user = ? ORDER BY filename")
        .bind(user)
        .map(|row: MySqlRow| row.get("filename"))
        .fetch_all(pool)
        .await?;
    Ok(q)
}

pub async fn get_users(pool: &MySqlPool) -> Result<Vec<String>> {
    let q: Vec<String> = sqlx::query("SELECT id FROM users ORDER BY id")
        .map(|row: MySqlRow| row.get("id"))
        .fetch_all(pool)
        .await?;
    Ok(q)
}

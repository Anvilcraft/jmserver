use sqlx::{postgres::PgRow, PgPool, Result, Row};

pub async fn get_cid(user: String, filename: String, pool: &PgPool) -> Result<String> {
    let q: String =
        sqlx::query("SELECT cid FROM memes WHERE userid = $1 AND filename = $2 ORDER BY id DESC")
            .bind(user)
            .bind(filename)
            .map(|row: PgRow| row.get("cid"))
            .fetch_one(pool)
            .await?;
    Ok(q)
}

pub async fn get_memes(user: String, pool: &PgPool) -> Result<Vec<String>> {
    let q: Vec<String> = sqlx::query("SELECT filename FROM memes WHERE userid = $1 ORDER BY filename")
        .bind(user)
        .map(|row: PgRow| row.get("filename"))
        .fetch_all(pool)
        .await?;
    Ok(q)
}

pub async fn get_users(pool: &PgPool) -> Result<Vec<String>> {
    let q: Vec<String> = sqlx::query("SELECT id FROM users ORDER BY id")
        .map(|row: PgRow| row.get("id"))
        .fetch_all(pool)
        .await?;
    Ok(q)
}

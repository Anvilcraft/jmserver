use sqlx::{MySqlPool, Result, Row, mysql::MySqlRow};


pub async fn get_cid(user: String, filename: String, pool: &MySqlPool) -> Result<String> {

    let q: String = sqlx::query("SELECT cid FROM memes WHERE user = ? AND filename = ? ORDER BY id DESC").bind(user).bind(filename)
        .map(|row: MySqlRow| row.get("cid"))
        .fetch_one(pool).await?;
        Ok(q)

}
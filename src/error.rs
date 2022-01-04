use thiserror::Error;


#[derive(Error, Debug)]
pub enum JMError {
    #[error("File read error: {0}")]
    Read(#[from] std::io::Error),
    #[error("Deserialize error: {0}")]
    Deserialize(#[from] toml::de::Error),
    #[error("Database connection error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Axum error: {0}")]
    Axum(#[from] hyper::Error),
}
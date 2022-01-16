mod error;
pub mod models;
mod routes;

use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts};
pub use routes::routes;
use serde::de::DeserializeOwned;

use self::error::APIError;

pub struct Query<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for Query<T>
where
    T: DeserializeOwned,
    B: Send,
{
    type Rejection = APIError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let query = axum::extract::Query::<T>::from_request(req).await?;
        Ok(Self(query.0))
    }
}

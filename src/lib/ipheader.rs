use std::{net::IpAddr, str::FromStr};

use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts};

use super::error::ExtractError;

pub struct ExtractIP(pub IpAddr);

#[async_trait]
impl<B> FromRequest<B> for ExtractIP
where
    B: Send,
{
    type Rejection = ExtractError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let header = req
            .headers()
            .and_then(|headers| headers.get("x-forwarded-for"));
        let header = header.ok_or(ExtractError::HeaderMissing("X-Forwarded-For".to_string()))?;
        let mut value = header.to_str()?;
        let pos = value.chars().position(|r| r == ',');
        value = match pos {
            Some(p) => &value[0..p],
            None => value,
        };
        let ip = IpAddr::from_str(value)?;

        Ok(Self(ip))
    }
}

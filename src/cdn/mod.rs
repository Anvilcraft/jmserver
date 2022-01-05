use axum::{
    body::Body,
    extract::{Extension, Path},
    handler::get,
    http::HeaderMap,
    response::IntoResponse,
    routing::BoxRoute,
    Router,
};
use headers::{ContentType, HeaderMapExt};
use reqwest::{
    header::{HeaderName, CONTENT_LENGTH},
    StatusCode,
};
use sqlx::MySqlPool;

use crate::config::ConfVars;

use self::{
    error::CDNError,
    templates::{DirTemplate, HtmlTemplate},
};

mod error;
mod sql;
mod templates;

pub fn routes() -> Router<BoxRoute> {
    Router::new()
        .route("/", get(users))
        .route("/:user/", get(memes))
        .route("/:user/:filename", get(image))
        .boxed()
}

async fn image(
    Path((user, filename)): Path<(String, String)>,
    Extension(db_pool): Extension<MySqlPool>,
    Extension(vars): Extension<ConfVars>,
) -> Result<impl IntoResponse, CDNError> {
    let filename = urlencoding::decode(&filename)?.into_owned();
    let cid = sql::get_cid(user, filename.clone(), &db_pool).await?;
    let ipfs = vars.ipfs_client()?;
    let res = ipfs.cat(cid).await?;
    let clength = res
        .headers()
        .get(HeaderName::from_static("x-content-length"))
        .ok_or(CDNError::Internal)?;

    let mut headers = HeaderMap::new();
    let ctype = ContentType::from(new_mime_guess::from_path(filename).first_or_octet_stream());
    headers.typed_insert(ctype);
    headers.insert(CONTENT_LENGTH, clength.clone());

    Ok((
        StatusCode::OK,
        headers,
        Body::wrap_stream(res.bytes_stream()),
    ))
}

async fn users(
    Extension(db_pool): Extension<MySqlPool>,
    Extension(vars): Extension<ConfVars>,
) -> Result<impl IntoResponse, CDNError> {
    let users = sql::get_users(&db_pool).await?;
    Ok(HtmlTemplate(DirTemplate {
        entries: users,
        prefix: vars.cdn,
        suffix: "/".to_string(),
    }))
}

async fn memes(
    Path(user): Path<String>,
    Extension(db_pool): Extension<MySqlPool>,
) -> Result<impl IntoResponse, CDNError> {
    let memes = sql::get_memes(user, &db_pool).await?;
    Ok(HtmlTemplate(DirTemplate {
        entries: memes,
        prefix: ".".to_string(),
        suffix: "".to_string(),
    }))
}

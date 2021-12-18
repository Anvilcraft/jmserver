
use axum::{Router, body::Body, extract::{Extension, Path}, handler::get, http::HeaderMap, response::IntoResponse, routing::BoxRoute};
use headers::{ContentType, HeaderMapExt};
use reqwest::{StatusCode, header::{CONTENT_LENGTH, HeaderName}};
use sqlx::{Error, MySqlPool};

use crate::config::ConfVars;

use self::templates::{DirTemplate, HtmlTemplate};

mod sql;
mod templates;

pub fn routes() -> Router<BoxRoute> {
    Router::new()
    .route("/", get(users))
    .route("/:user/", get(memes))
    .route("/:user/:filename", get(image))
    .boxed()
}

async fn image(Path((user, filename)): Path<(String, String)>, Extension(db_pool): Extension<MySqlPool>, Extension(vars): Extension<ConfVars>) -> Result<impl IntoResponse, StatusCode> {
    let q = sql::get_cid(user, filename.clone(), &db_pool).await;
    match q {
        Ok(cid) => {
            let ipfsapi = vars.ipfs_client();
            match ipfsapi {
                Ok(ipfs) => {
                    let res = ipfs.cat(cid).await;
                    match res {
                        Ok(r) => {
                            let clength = r.headers().get(HeaderName::from_static("x-content-length"));
                            match clength {
                                Some(h) => {
                                    let mut headers = HeaderMap::new();
                                    let ctype = ContentType::from(new_mime_guess::from_path(filename).first_or_octet_stream());
                                    headers.typed_insert(ctype);
                                    headers.insert(CONTENT_LENGTH, h.clone());    
                                    
                                    Ok((StatusCode::OK, headers, Body::wrap_stream(r.bytes_stream())))
                                },
                                None => Err(StatusCode::INTERNAL_SERVER_ERROR),
                            }
                        },
                        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                    }
                },
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        },
        Err(err) => match err {
            Error::RowNotFound => Err(StatusCode::NOT_FOUND),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

async fn users(Extension(db_pool): Extension<MySqlPool>, Extension(vars): Extension<ConfVars>) -> Result<impl IntoResponse, StatusCode> {
    let q = sql::get_users(&db_pool).await;
    match q {
        Ok(users) => Ok(HtmlTemplate(DirTemplate { 
            entries: users,
            prefix: vars.cdn,
            suffix: "/".to_string(),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn memes(Path(user): Path<String>, Extension(db_pool): Extension<MySqlPool>) -> Result<impl IntoResponse, StatusCode> {
    let q = sql::get_memes(user, &db_pool).await;
    match q {
        Ok(memes) => Ok(HtmlTemplate(DirTemplate { 
            entries: memes,
            prefix: ".".to_string(),
            suffix: "".to_string(),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
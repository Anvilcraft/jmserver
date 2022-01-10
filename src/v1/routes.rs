use crate::config::ConfVars;
use crate::ipfs::IPFSFile;
use crate::lib::ExtractIP;
use crate::v1::models::*;

use axum::extract::{ContentLengthLimit, Extension, Multipart, Query};
use axum::handler::{get, post};
use axum::response::IntoResponse;
use axum::routing::BoxRoute;
use axum::{Json, Router};
use hyper::StatusCode;
use sqlx::MySqlPool;

use super::error::APIError;

async fn meme(
    params: Query<MemeIDQuery>,
    Extension(db_pool): Extension<MySqlPool>,
    Extension(vars): Extension<ConfVars>,
) -> Result<impl IntoResponse, APIError> {
    let meme = Meme::get(params.id, &db_pool, vars.cdn).await?;
    Ok(Json(MemeResponse {
        status: 200,
        error: None,
        meme: Some(meme),
    }))
}

async fn memes(
    params: Query<MemeFilterQuery>,
    Extension(db_pool): Extension<MySqlPool>,
    Extension(vars): Extension<ConfVars>,
) -> Result<impl IntoResponse, APIError> {
    let memes = Meme::get_all(params.0, &db_pool, vars.cdn).await?;
    Ok(Json(MemesResponse {
        status: 200,
        error: None,
        memes: Some(memes),
    }))
}

async fn category(
    params: Query<IDQuery>,
    Extension(db_pool): Extension<MySqlPool>,
) -> Result<impl IntoResponse, APIError> {
    let category = Category::get(&params.id, &db_pool).await?;
    Ok(Json(CategoryResponse {
        status: 200,
        error: None,
        category: Some(category),
    }))
}

async fn categories(
    Extension(db_pool): Extension<MySqlPool>,
) -> Result<impl IntoResponse, APIError> {
    let categories = Category::get_all(&db_pool).await?;
    Ok(Json(CategoriesResponse {
        status: 200,
        error: None,
        categories: Some(categories),
    }))
}

async fn user(
    params: Query<UserIDQuery>,
    Extension(db_pool): Extension<MySqlPool>,
) -> Result<impl IntoResponse, APIError> {
    let user = User::get(params.0, &db_pool).await?;
    Ok(Json(UserResponse {
        status: 200,
        error: None,
        user: Some(user),
    }))
}

async fn users(Extension(db_pool): Extension<MySqlPool>) -> Result<impl IntoResponse, APIError> {
    let users = User::get_all(&db_pool).await?;
    Ok(Json(UsersResponse {
        status: 200,
        error: None,
        users: Some(users),
    }))
}

async fn random(
    params: Query<MemeFilterQuery>,
    Extension(db_pool): Extension<MySqlPool>,
    Extension(vars): Extension<ConfVars>,
) -> Result<impl IntoResponse, APIError> {
    let random = Meme::get_random(params.0, &db_pool, vars.cdn).await?;
    Ok(Json(MemeResponse {
        status: 200,
        error: None,
        meme: Some(random),
    }))
}

async fn upload(
    ContentLengthLimit(mut form): ContentLengthLimit<Multipart, { 1024 * 1024 * 1024 }>,
    Extension(db_pool): Extension<MySqlPool>,
    Extension(vars): Extension<ConfVars>,
    ExtractIP(ip): ExtractIP,
) -> Result<impl IntoResponse, APIError> {
    let mut category: Option<String> = None;
    let mut token: Option<String> = None;
    let mut files: Vec<IPFSFile> = vec![];

    let ipfs = vars.ipfs_client()?;

    while let Some(field) = form.next_field().await? {
        match field.name().ok_or_else(|| {
            APIError::BadRequest("A multipart-form field is missing a name".to_string())
        })? {
            "token" => token = Some(field.text().await?),
            "category" => category = Some(field.text().await?),
            "file" | "file[]" => {
                let filename = field
                    .file_name()
                    .ok_or_else(|| {
                        APIError::BadRequest("A file field has no filename".to_string())
                    })?
                    .to_string();
                let file = ipfs.add(field.bytes().await?, filename).await?;
                files.push(file);
            }
            _ => (),
        }
    }

    let token = token.ok_or_else(|| APIError::Unauthorized("Missing token".to_string()))?;
    let category = category.ok_or_else(|| APIError::BadRequest("Missing category".to_string()))?;
    let user = User::check_token(token, &db_pool)
        .await?
        .ok_or_else(|| APIError::Forbidden("token not existing".to_string()))?;
    let total = (user.dayuploads as isize) + (files.len() as isize);

    if total > 20 {
        return Err(APIError::Forbidden("Upload limit reached".to_string()));
    }

    let cat = Category::get(&category, &db_pool).await?;

    let ip = ip.to_string();

    let mut links: Vec<String> = vec![];

    for f in files {
        let res = cat.add_meme(&user, &f, &ip, &db_pool).await?;

        if res != 1 {
            return Err(APIError::Internal("Database insertion error".to_string()));
        }

        ipfs.pin(f.hash).await?;
        links.push(format!(
            "{}/{}/{}",
            vars.cdn,
            user.id.clone(),
            f.name.clone()
        ));
    }

    Ok((
        StatusCode::CREATED,
        Json(UploadResponse {
            status: 201,
            error: None,
            files: Some(links),
        }),
    ))
}

//TODO: Implement upload endpoint

pub fn routes() -> Router<BoxRoute> {
    Router::new()
        .route("/meme", get(meme))
        .route("/memes", get(memes))
        .route("/category", get(category))
        .route("/categories", get(categories))
        .route("/user", get(user))
        .route("/users", get(users))
        .route("/random", get(random))
        .route("/upload", post(upload))
        .boxed()
}

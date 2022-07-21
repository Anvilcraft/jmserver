use crate::ipfs::IPFSFile;
use crate::lib::ExtractIP;
use crate::v1::models::*;
use crate::JMService;

use axum::extract::{ContentLengthLimit, Extension, Multipart};
use axum::handler::{get, post};
use axum::response::IntoResponse;
use axum::routing::BoxRoute;
use axum::{Json, Router};
use hyper::StatusCode;

use super::Query;
use crate::error::APIError;

async fn meme(
    Query(params): Query<MemeIDQuery>,
    Extension(service): Extension<JMService>,
) -> Result<impl IntoResponse, APIError> {
    let meme = V1Meme::new(
        service
            .get_meme(params.id)
            .await?
            .ok_or_else(|| APIError::NotFound("Meme not found".to_string()))?,
        service.cdn_url(),
    );
    Ok(Json(MemeResponse {
        status: 200,
        error: None,
        meme: Some(meme),
    }))
}

async fn memes(
    Query(params): Query<MemeFilter>,
    Extension(service): Extension<JMService>,
) -> Result<impl IntoResponse, APIError> {
    let memes = service
        .get_memes(params.into())
        .await?
        .into_iter()
        .map(|meme| V1Meme::new(meme, service.cdn_url()))
        .collect();
    Ok(Json(MemesResponse {
        status: 200,
        error: None,
        memes: Some(memes),
    }))
}

async fn category(
    Query(params): Query<IDQuery>,
    Extension(service): Extension<JMService>,
) -> Result<impl IntoResponse, APIError> {
    let category = service
        .get_category(&params.id)
        .await?
        .ok_or_else(|| APIError::NotFound("Category not found".to_string()))?;
    Ok(Json(CategoryResponse {
        status: 200,
        error: None,
        category: Some(category),
    }))
}

async fn categories(
    Extension(service): Extension<JMService>,
) -> Result<impl IntoResponse, APIError> {
    let categories = service.get_categories().await?;
    Ok(Json(CategoriesResponse {
        status: 200,
        error: None,
        categories: Some(categories),
    }))
}

async fn user(
    Query(params): Query<UserIDQuery>,
    Extension(service): Extension<JMService>,
) -> Result<impl IntoResponse, APIError> {
    let user = service
        .get_user(params.into())
        .await?
        .ok_or_else(|| APIError::NotFound("User not found".to_string()))?;
    Ok(Json(UserResponse {
        status: 200,
        error: None,
        user: Some(user),
    }))
}

async fn users(Extension(service): Extension<JMService>) -> Result<impl IntoResponse, APIError> {
    let users = service.get_users().await?;
    Ok(Json(UsersResponse {
        status: 200,
        error: None,
        users: Some(users),
    }))
}

async fn random(
    Query(params): Query<MemeFilter>,
    Extension(service): Extension<JMService>,
) -> Result<impl IntoResponse, APIError> {
    let random = V1Meme::new(
        service.get_random_meme(params.into()).await?,
        service.cdn_url(),
    );
    Ok(Json(MemeResponse {
        status: 200,
        error: None,
        meme: Some(random),
    }))
}

async fn upload(
    ContentLengthLimit(mut form): ContentLengthLimit<Multipart, { 1024 * 1024 * 1024 }>,
    Extension(service): Extension<JMService>,
    ExtractIP(ip): ExtractIP,
) -> Result<impl IntoResponse, APIError> {
    let mut category: Option<String> = None;
    let mut token: Option<String> = None;
    let mut files: Vec<IPFSFile> = vec![];

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
                let file = service.ipfs_add(field.bytes().await?, filename).await?;
                files.push(file);
            },
            _ => (),
        }
    }

    let token = token.ok_or_else(|| APIError::Unauthorized("Missing token".to_string()))?;
    let category = category.ok_or_else(|| APIError::BadRequest("Missing category".to_string()))?;
    let user = service
        .check_token(&token)
        .await?
        .ok_or_else(|| APIError::Forbidden("token not existing".to_string()))?;
    let total = (user.dayuploads as isize) + (files.len() as isize);

    if total > 20 {
        return Err(APIError::Forbidden("Upload limit reached".to_string()));
    }

    let cat = service
        .get_category(&category)
        .await?
        .ok_or_else(|| APIError::BadRequest("Category not existing".to_string()))?;

    let ip = ip.to_string();

    let mut links: Vec<String> = vec![];

    for f in files {
        let res = service.add_meme_sql(&user, &f, &ip, &cat).await?;

        if res == 0 {
            return Err(APIError::Internal("Database insertion error".to_string()));
        }
        service
            .add_meme(
                cat.id.clone(),
                f.name.clone(),
                f.hash.clone(),
                user.id.clone(),
                res,
            )
            .await?;
        service.ipfs_pin(f.hash).await?;
        links.push(format!(
            "{}/{}/{}",
            service.cdn_url(),
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
            token,
        }),
    ))
}

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

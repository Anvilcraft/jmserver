use axum::{
    extract::{Extension, Path, Query},
    handler::get,
    response::IntoResponse,
    routing::BoxRoute,
    Json, Router,
};

use crate::{
    error::APIError,
    models::{MemeOptions, UserIdentifier},
    JMService,
};

use super::models::{V2Meme, V2User};

async fn get_meme(
    Path(meme_id): Path<i32>,
    Extension(service): Extension<JMService>,
) -> Result<impl IntoResponse, APIError> {
    Ok(Json(V2Meme::from(
        service
            .get_meme(meme_id)
            .await?
            .ok_or_else(|| APIError::NotFound("Meme not found".to_string()))?,
    )))
}

async fn get_memes(
    Extension(service): Extension<JMService>,
) -> Result<impl IntoResponse, APIError> {
    Ok(Json(
        service
            .get_memes(MemeOptions::empty())
            .await?
            .into_iter()
            .map(V2Meme::from)
            .collect::<Vec<V2Meme>>(),
    ))
}

async fn get_category(
    Path(category_id): Path<String>,
    Extension(service): Extension<JMService>,
) -> Result<impl IntoResponse, APIError> {
    Ok(Json(service.get_category(&category_id).await?.ok_or_else(
        || APIError::NotFound("Category not found".to_string()),
    )?))
}

async fn get_categories(
    Extension(service): Extension<JMService>,
) -> Result<impl IntoResponse, APIError> {
    Ok(Json(service.get_categories().await?))
}

async fn get_user(
    Path(user_id): Path<String>,
    Extension(service): Extension<JMService>,
) -> Result<impl IntoResponse, APIError> {
    Ok(Json(V2User::from(
        service
            .get_user(UserIdentifier::Id(user_id))
            .await?
            .ok_or_else(|| APIError::NotFound("User not found".to_string()))?,
    )))
}

async fn get_users(
    Extension(service): Extension<JMService>,
) -> Result<impl IntoResponse, APIError> {
    Ok(Json(
        service
            .get_users()
            .await?
            .into_iter()
            .map(V2User::from)
            .collect::<Vec<V2User>>(),
    ))
}

async fn get_user_memes(
    Path(user_id): Path<String>,
    Extension(service): Extension<JMService>,
) -> Result<impl IntoResponse, APIError> {
    Ok(Json(
        service
            .get_memes(MemeOptions {
                category: None,
                user_id: Some(user_id),
                username: None,
                search: None,
                limit: None,
                after: None,
            })
            .await?
            .into_iter()
            .map(V2Meme::from)
            .collect::<Vec<V2Meme>>(),
    ))
}

async fn get_user_meme(
    Path((user_id, filename)): Path<(String, String)>,
    Extension(service): Extension<JMService>,
) -> Result<impl IntoResponse, APIError> {
    let decoded = urlencoding::decode(&filename)?.into_owned();
    Ok(Json(V2Meme::from(
        service
            .get_user_meme(user_id, decoded)
            .await?
            .ok_or_else(|| APIError::NotFound("Meme not found".to_string()))?,
    )))
}

pub fn routes() -> Router<BoxRoute> {
    Router::new()
        .route("/memes", get(get_memes))
        .route("/memes/:meme_id", get(get_meme))
        .route("/categories", get(get_categories))
        .route("/categories/:category_id", get(get_category))
        .route("/users", get(get_users))
        .route("/users/:user_id", get(get_user))
        .route("/users/:user_id/memes", get(get_user_memes))
        .route("/users/:user_id/memes/:filename", get(get_user_meme))
        .boxed()
}

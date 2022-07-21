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

use super::models::{MemeFilterQuery, V2Meme, V2User};

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
    Query(filter): Query<MemeFilterQuery>,
    Extension(service): Extension<JMService>,
) -> Result<impl IntoResponse, APIError> {
    Ok(Json(
        service
            .get_memes(filter.into())
            .await?
            .into_iter()
            .map(V2Meme::from)
            .collect::<Vec<V2Meme>>(),
    ))
}

async fn get_random_meme(
    Query(filter): Query<MemeFilterQuery>,
    Extension(service): Extension<JMService>,
) -> Result<impl IntoResponse, APIError> {
    Ok(Json(V2Meme::from(
        service.get_random_meme(filter.into()).await?,
    )))
}

async fn count_memes(
    Query(filter): Query<MemeFilterQuery>,
    Extension(service): Extension<JMService>,
) -> Result<impl IntoResponse, APIError> {
    Ok(Json(service.count_memes(filter.into()).await?))
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
    Query(filter): Query<MemeFilterQuery>,
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
                limit: filter.limit,
                after: filter.after,
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

fn meme_routes() -> Router<BoxRoute> {
    Router::new()
        .route("/", get(get_memes))
        .route("/:meme_id", get(get_meme))
        .route("/random", get(get_random_meme))
        .route("/count", get(count_memes))
        .boxed()
}

fn category_routes() -> Router<BoxRoute> {
    Router::new()
        .route("/", get(get_categories))
        .route("/:category_id", get(get_category))
        .boxed()
}

fn user_routes() -> Router<BoxRoute> {
    Router::new()
        .route("/", get(get_users))
        .route("/:user_id", get(get_user))
        .route("/:user_id/memes", get(get_user_memes))
        .route("/:user_id/memes/:filename", get(get_user_meme))
        .boxed()
}

pub fn routes() -> Router<BoxRoute> {
    Router::new()
        .nest("/memes", meme_routes())
        .nest("/categories", category_routes())
        .nest("/users", user_routes())
        .boxed()
}

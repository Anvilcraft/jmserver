use crate::config::ConfVars;
use crate::v1::models::*;
use axum::extract::{ContentLengthLimit, Extension, Multipart, Query};
use axum::handler::{get, post};
use axum::response::IntoResponse;
use axum::routing::BoxRoute;
use axum::{Json, Router};
use sqlx::MySqlPool;

async fn meme(
    params: Query<MemeIDQuery>,
    Extension(db_pool): Extension<MySqlPool>,
    Extension(vars): Extension<ConfVars>,
) -> Result<impl IntoResponse, ErrorResponse> {
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
) -> Result<impl IntoResponse, ErrorResponse> {
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
) -> Result<impl IntoResponse, ErrorResponse> {
    let category = Category::get(&params.id, &db_pool).await?;
    Ok(Json(CategoryResponse {
        status: 200,
        error: None,
        category: Some(category),
    }))
}

async fn categories(
    Extension(db_pool): Extension<MySqlPool>,
) -> Result<impl IntoResponse, ErrorResponse> {
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
) -> Result<impl IntoResponse, ErrorResponse> {
    let user = User::get(params.0, &db_pool).await?;
    Ok(Json(UserResponse {
        status: 200,
        error: None,
        user: Some(user),
    }))
}

async fn users(
    Extension(db_pool): Extension<MySqlPool>,
) -> Result<impl IntoResponse, ErrorResponse> {
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
) -> Result<impl IntoResponse, ErrorResponse> {
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
) -> impl IntoResponse {
    todo!();
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

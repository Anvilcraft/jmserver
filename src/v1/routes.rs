use crate::config::ConfVars;
use crate::v1::models::*;
use sqlx::{MySqlPool, Error};
use axum::{Router, Json};
use axum::routing::BoxRoute;
use axum::response::IntoResponse;
use axum::handler::get;
use axum::extract::{Query, Extension};
use axum::http::StatusCode;

async fn meme(params: Query<MemeIDQuery>, Extension(db_pool): Extension<MySqlPool>, Extension(vars): Extension<ConfVars>) -> impl IntoResponse {
    let q = Meme::get(params.id, &db_pool, vars.cdn).await;
    match q {
        Ok(meme) => (StatusCode::OK, Json(MemeResponse {
            status: 200,
            error: None,
            meme: Some(meme)
        })),
        Err(err) => match err {
            Error::RowNotFound => (StatusCode::NOT_FOUND, Json(MemeResponse {
                status: 404,
                error: Some(String::from("Meme not found")),
                meme: None
            })),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(MemeResponse {
                status: 500,
                error: Some(String::from("Internal Server Error")),
                meme: None
            }))
        }
    }
}

async fn memes(params: Query<MemeFilterQuery>, Extension(db_pool): Extension<MySqlPool>, Extension(vars): Extension<ConfVars>) -> impl IntoResponse {
    let q = Meme::get_all(params.0, &db_pool, vars.cdn).await;
    match q {
        Ok(memes) => (StatusCode::OK, Json(MemesResponse {
            status: 200,
            error: None,
            memes: Some(memes)
        })),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(MemesResponse {
            status: 500,
            error: Some(String::from("Internal Server Error")),
            memes: None
        }))
    }
}

async fn category(params: Query<IDQuery>, Extension(db_pool): Extension<MySqlPool>) -> impl IntoResponse {
    let q = Category::get(&params.id, &db_pool).await;
    match q {
        Ok(category) => (StatusCode::OK, Json(CategoryResponse { status: 200, error: None, category: Some(category)})),
        Err(err) => match err {
            Error::RowNotFound => (StatusCode::NOT_FOUND, Json(CategoryResponse {
                status: 404,
                error: Some(String::from("Category not found")),
                category: None
            })),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(CategoryResponse {
                status: 500,
                error: Some(String::from("Internal Server Error")),
                category: None
            }))
        }
    }
}

async fn categories(Extension(db_pool): Extension<MySqlPool>) -> impl IntoResponse {
    let q = Category::get_all(&db_pool).await;
    match q {
        Ok(categories) => (StatusCode::OK, Json(CategoriesResponse { status: 200, error: None, categories: Some(categories)})),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(CategoriesResponse {
            status: 500,
            error: Some(String::from("Internal Server Error")),
            categories: None
        }))
    }
}

async fn user(params: Query<UserIDQuery>, Extension(db_pool): Extension<MySqlPool>) -> impl IntoResponse {
    let q = User::get(params.0, &db_pool).await;
    match q {
        Ok(user) => (StatusCode::OK, Json(UserResponse { status: 200, error: None, user: Some(user)})),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(UserResponse {
            status: 500,
            error: Some(String::from("Internal Server Error")),
            user: None
        }))
    }
}

async fn users(Extension(db_pool): Extension<MySqlPool>) -> impl IntoResponse {
    let q = User::get_all(&db_pool).await;
    match q {
        Ok(users) => (StatusCode::OK, Json(UsersResponse { status: 200, error: None, users: Some(users)})),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(UsersResponse {
            status: 500,
            error: Some(String::from("Internal Server Error")),
            users: None
        }))
    }
}

async fn random(params: Query<MemeFilterQuery>, Extension(db_pool): Extension<MySqlPool>, Extension(vars): Extension<ConfVars>) -> impl IntoResponse {
    let q = Meme::get_random(params.0, &db_pool, vars.cdn).await;
    match q {
        Ok(random) => (StatusCode::OK, Json(MemeResponse {
            status: 200,
            error: None,
            meme: Some(random)
        })),
        Err(err) => match err {
            Error::RowNotFound => (StatusCode::NOT_FOUND, Json(MemeResponse {
                status: 404,
                error: Some(String::from("Meme not found")),
                meme: None
            })),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(MemeResponse {
                status: 500,
                error: Some(String::from("Internal Server Error")),
                meme: None
            }))
        }
    }
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
        .boxed()
}

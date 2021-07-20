use actix_web::{web, get, Responder, HttpResponse};
use crate::v1::models::*;
use sqlx::{MySqlPool, Error};

#[get("/v1/meme")]
async fn meme(params: web::Query<MemeIDQuery>, db_pool: web::Data<MySqlPool>) -> impl Responder {
    let q = Meme::get(params.id, db_pool.get_ref()).await;
    match q {
        Ok(meme) => HttpResponse::Ok().json(MemeResponse {
            status: 200,
            error: None,
            meme: Option::from(meme)
        }),
        Err(err) => match err {
            Error::RowNotFound => HttpResponse::NotFound().json(MemeResponse {
                status: 404,
                error: Option::from(String::from("Meme not found")),
                meme: None
            }),
            _ => HttpResponse::InternalServerError().json(MemeResponse {
                status: 500,
                error: Option::from(String::from("Internal Server Error")),
                meme: None
            })
        }
    }
}

#[get("/v1/memes")]
async fn memes(params: web::Query<MemeFilterQuery>, db_pool: web::Data<MySqlPool>) -> impl Responder {
    let q = Meme::get_all(params.0, db_pool.get_ref()).await;
    match q {
        Ok(memes) => HttpResponse::Ok().json(MemesResponse {
            status: 200,
            error: None,
            memes: Option::from(memes)
        }),
        _ => HttpResponse::InternalServerError().json(MemesResponse {
            status: 500,
            error: Option::from(String::from("Internal Server Error")),
            memes: None
        })
    }
}

#[get("/v1/category")]
async fn category(params: web::Query<IDQuery>, db_pool: web::Data<MySqlPool>) -> impl Responder {
    let q = Category::get(&params.id, db_pool.get_ref()).await;
    match q {
        Ok(category) => HttpResponse::Ok().json(CategoryResponse { status: 200, error: None, category: Option::from(category)}),
        Err(err) => match err {
            Error::RowNotFound => HttpResponse::NotFound().json(CategoryResponse {
                status: 404,
                error: Option::from(String::from("Category not found")),
                category: None
            }),
            _ => HttpResponse::InternalServerError().json(CategoryResponse {
                status: 500,
                error: Option::from(String::from("Internal Server Error")),
                category: None
            })
        }
    }
}

#[get("/v1/categories")]
async fn categories(db_pool: web::Data<MySqlPool>) -> impl Responder {
    let q = Category::get_all(db_pool.get_ref()).await;
    match q {
        Ok(categories) => HttpResponse::Ok().json(CategoriesResponse { status: 200, error: None, categories: Option::from(categories)}),
        _ => HttpResponse::InternalServerError().json(CategoriesResponse {
            status: 500,
            error: Option::from(String::from("Internal Server Error")),
            categories: None
        })
    }
}

#[get("/v1/user")]
async fn user(params: web::Query<UserIDQuery>, db_pool: web::Data<MySqlPool>) -> impl Responder {
    let q = User::get(params.0,db_pool.get_ref()).await;
    match q {
        Ok(user) => HttpResponse::Ok().json(UserResponse { status: 200, error: None, user: Option::from(user)}),
        _ => HttpResponse::InternalServerError().json(UserResponse {
            status: 500,
            error: Option::from(String::from("Internal Server Error")),
            user: None
        })
    }
}

#[get("/v1/users")]
async fn users(db_pool: web::Data<MySqlPool>) -> impl Responder {
    let q = User::get_all(db_pool.get_ref()).await;
    match q {
        Ok(users) => HttpResponse::Ok().json(UsersResponse { status: 200, error: None, users: Option::from(users)}),
        _ => HttpResponse::InternalServerError().json(UsersResponse {
            status: 500,
            error: Option::from(String::from("Internal Server Error")),
            users: None
        })
    }
}

#[get("/v1/random")]
async fn random(params: web::Query<MemeFilterQuery>, db_pool: web::Data<MySqlPool>) -> impl Responder {
    let q = Meme::get_random(params.0, db_pool.get_ref()).await;
    match q {
        Ok(random) => HttpResponse::Ok().json(MemeResponse {
            status: 200,
            error: None,
            meme: Some(random)
        }),
        Err(err) => match err {
            Error::RowNotFound => HttpResponse::NotFound().json(MemeResponse {
                status: 404,
                error: Some(String::from("Meme not found")),
                meme: None
            }),
            _ => HttpResponse::InternalServerError().json(MemeResponse {
                status: 500,
                error: Some(String::from("Internal Server Error")),
                meme: None
            })
        }
    }
}

//TODO: Implement random meme endpoint
//TODO: Implement upload endpoint

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(meme);
    cfg.service(memes);
    cfg.service(category);
    cfg.service(categories);
    cfg.service(user);
    cfg.service(users);
    cfg.service(random);
}

use std::{io, env};
use sqlx::MySqlPool;
use std::net::SocketAddr;
use axum::Router;
use tower_http::add_extension::AddExtensionLayer;

mod v1;

#[tokio::main]
async fn main() {

    let database_url = env::var("DBURL").unwrap();
    let db_pool = MySqlPool::new(&database_url).await.unwrap();

    let app = Router::new()
        .nest("/v1", v1::routes())
        .layer(AddExtensionLayer::new(db_pool));

    let addr: SocketAddr = env::var("LISTEN").expect("The LISTEN env var ist not set").parse().expect("The LISTEN env var is set incorrectly");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Something went wrong :(");
}

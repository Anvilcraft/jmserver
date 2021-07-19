use actix_web::{HttpServer, App};
use std::{io, env};
use sqlx::MySqlPool;

mod v1;

#[actix_web::main]
async fn main() -> io::Result<()>{

    let database_url = env::var("DBURL").unwrap();
    let db_pool = MySqlPool::new(&database_url).await.unwrap();

    let mut server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .configure(v1::init)
    });

    server = server.bind(env::var("LISTEN").unwrap())?;
    server.run().await
}

use std::{env, io, path::PathBuf};
use config::Config;
use sqlx::MySqlPool;
use structopt::StructOpt;
use axum::{Router, body::Body, http::{HeaderValue, Request, header}};
use tower_http::{add_extension::AddExtensionLayer, set_header::SetResponseHeaderLayer};

mod v1;
mod config;

#[derive(StructOpt)]
struct Opt {
    #[structopt(
    short,
    long,
    help = "config file to use",
    default_value = "./config.toml"
    )]
    config: PathBuf,
}

#[tokio::main]
async fn main() {

    let opt = Opt::from_args();
    let config = std::fs::read(&opt.config).expect("Config file reading error");
    let config = toml::from_slice::<Config>(&config).expect("Config file parsing error");

    let db_pool = MySqlPool::new(&config.database).await.expect("Database connection error");

    let app = Router::new()
        .nest("/v1", v1::routes())
        .layer(AddExtensionLayer::new(db_pool))
        .layer(AddExtensionLayer::new(config.vars()))
        .layer(SetResponseHeaderLayer::<_, Request<Body>>::if_not_present(header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*")));

    axum::Server::bind(&config.addr)
        .serve(app.into_make_service())
        .await
        .expect("Something went wrong :(");
}


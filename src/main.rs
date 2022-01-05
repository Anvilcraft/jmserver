use axum::{
    body::Body,
    http::{header, HeaderValue, Request},
    Router,
};
use config::Config;
use error::JMError;
use sqlx::MySqlPool;
use std::path::PathBuf;
use structopt::StructOpt;
use tower_http::{add_extension::AddExtensionLayer, set_header::SetResponseHeaderLayer};

mod cdn;
mod config;
mod error;
mod ipfs;
mod v1;

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
async fn main() -> Result<(), JMError> {
    let opt = Opt::from_args();
    let config = std::fs::read(&opt.config)?;
    let config = toml::from_slice::<Config>(&config)?;

    let db_pool = MySqlPool::new(&config.database).await?;

    let app = Router::new()
        .nest("/api/v1", v1::routes())
        .nest("/cdn", cdn::routes())
        .layer(AddExtensionLayer::new(db_pool))
        .layer(AddExtensionLayer::new(config.vars()))
        .layer(SetResponseHeaderLayer::<_, Request<Body>>::if_not_present(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        ));

    axum::Server::bind(&config.addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

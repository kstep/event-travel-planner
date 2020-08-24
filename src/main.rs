#![deny(dead_code)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

mod db;
mod handlers;
mod models;
mod result;
mod schema;

use crate::result::Result;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

#[derive(Deserialize)]
struct Config {
    database_url: String,
    http_host: String,
}

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    env_logger::init();

    let config: Config = envy::from_env()?;
    let pool = db::create_pool(&*config.database_url)?;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .configure(handlers::setup)
    })
    .bind(&*config.http_host)
    .map(|server| {
        info!("Running server http://{}...", &*config.http_host);
        server
    })?
    .run()
    .await?;

    Ok(())
}

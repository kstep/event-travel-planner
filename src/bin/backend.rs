#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use event_travel_planner::backend::db::create_pool;
use event_travel_planner::backend::{handlers, result};

#[derive(Deserialize)]
struct Config {
    database_url: String,
    http_host: String,
}

#[actix_rt::main]
async fn main() -> result::Result<()> {
    dotenv::dotenv()?;
    env_logger::init();

    let config: Config = envy::from_env()?;
    let pool = create_pool(&*config.database_url)?;
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .configure(handlers::setup)
            .service(Files::new("/", "./static/").index_file("index.html"))
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

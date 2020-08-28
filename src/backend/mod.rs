use crate::backend::db::create_pool;
use actix_files::Files;
use actix_web::body::Body;
use actix_web::middleware::Logger;
use actix_web::{App, HttpResponse, HttpServer};

mod db;
mod handlers;
mod result;
pub mod schema;

#[derive(Deserialize)]
struct Config {
    database_url: String,
    http_host: String,
}

fn redirect_to_root() -> HttpResponse<Body> {
    HttpResponse::TemporaryRedirect()
        .header(actix_web::http::header::LOCATION, "/")
        .finish()
}

#[actix_rt::main]
pub async fn run_backend() -> result::Result<()> {
    dotenv::dotenv()?;
    env_logger::init();

    let config: Config = envy::from_env()?;
    let pool = create_pool(&*config.database_url)?;
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .configure(handlers::setup)
            .service(
                Files::new("/", "./static/")
                    .index_file("index.html")
                    .default_handler(actix_web::web::get().to(redirect_to_root)),
            )
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

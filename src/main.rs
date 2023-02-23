use crate::config::AppConfig;
use ::config::Config;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenvy::dotenv;
use env_logger;
use tokio_postgres::NoTls;

mod config;
mod db;
mod errors;
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: AppConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(logger)
            .service(web::resource("/l").route(web::post().to(handlers::shorten)))
            .service(web::resource("/l/{slug}").route(web::get().to(handlers::redirect)))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

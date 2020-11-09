#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate diesel_migrations;
use actix_web::{App, HttpServer};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
mod blog;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    HttpServer::new(move || {
        App::new()
            .data(config::db::migrate_and_config_db(&database_url))
            .configure(blog::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


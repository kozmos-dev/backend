#![feature(lazy_cell)]

pub mod database;

pub mod auth;
pub mod config;
#[macro_use]
pub mod utils;

//use actix_files::{Files, NamedFile};
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let config = config::CONFIG.read().unwrap();

    HttpServer::new(|| {
        App::new()
    })
    .bind((config.web_server.address.as_ref(), config.web_server.port))?
    .run()
    .await
}

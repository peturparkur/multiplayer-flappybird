use actix_files::{Files, NamedFile};
use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpServer, Responder};
use actix_web_actors::ws;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    return Ok(())
}

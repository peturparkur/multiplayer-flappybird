mod game_lobby;
use std::sync::Arc;

use actix_files::{Files, NamedFile};
// use lobby::Lobby;
use game_lobby::Lobby;
mod endpoints;
mod game;
mod messages;
mod socket;
use actix::Actor;
use actix_web::{middleware::Logger, web::{Data, self}, App, HttpServer, Responder};

use crate::socket::WsConn;
use endpoints::start_connection as start_connection_route;


async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let chat_server = Data::new(Lobby::<WsConn>::default().start()); //create and spin up a lobby

    let server = HttpServer::new(move || {
        App::new()
            .service(web::resource("/{group_id}").to(start_connection_route)) //. rename with "as" import or naming conflict
            .app_data(chat_server.clone()) //register the lobby
            .service(web::resource("/").to(index)) // serve the index function as the default root
            .service(Files::new("/static", "./static")) // serve the files in /static folder
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run();

    println!("server running on http://localhost:8080");
    server.await
}

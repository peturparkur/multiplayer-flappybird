use actix::dev::ToEnvelope;
use actix::{Actor, Addr};
use actix_web::{get, web::Data, web::Path, web::Payload, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;

// use crate::lobby::Lobby;
use crate::game_lobby::Lobby;
use crate::messages::{Connect, Disconnect, WsMessage};
use crate::socket::WsConn;

pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    _path: Path<u64>,
    srv: Data<Addr<Lobby<WsConn>>>,
) -> Result<HttpResponse, Error> {
    println!("Called endpoint!");

    let group_id = _path.into_inner();

    let ws = WsConn::new(srv.get_ref().clone(), Some(group_id as u128));

    let resp = match ws::start(ws, &req, stream) {
        Ok(resp) => resp,
        Err(e) => {
            println!("WebSocket error: {}", e);
            HttpResponse::InternalServerError().json(e.to_string())
        }
    };
    Ok(resp)
}

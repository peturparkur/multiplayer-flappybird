use actix::{
    dev::ToEnvelope, fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext,
    ContextFutureSpawner, Handler, Message, Running, StreamHandler, WrapFuture,
};
use actix_web_actors::ws;
use actix_web_actors::ws::Message::Text;
use std::time::{Duration, Instant};
use uuid::Uuid;

use crate::{
    // lobby::Lobby,
    game_lobby::Lobby,
    messages::{ClientActorMessage, Connect, Disconnect, WsMessage},
};

// Will need an actor to handle the websocket connection.
// use actix::Addr; // Addr is an alias for Addr<Actor>

pub struct WsConn {
    pub room: u128,
    pub lobby_addr: Addr<Lobby<WsConn>>,
    pub hb: Instant,
    pub id: u128,
}

impl WsConn {
    pub fn new(lobby_addr: Addr<Lobby<WsConn>>, room: Option<u128>) -> WsConn {
        WsConn {
            room: room.unwrap_or(0),
            lobby_addr,
            hb: Instant::now(),
            id: Uuid::new_v4().as_u128(),
        }
    }
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    // what to do when connection is opened
    fn started(&mut self, ctx: &mut Self::Context) {
        // Spawn heartbeat job
        self.spawn_heartbeat_job(ctx);

        // Connect to main lobby
        let addr = ctx.address(); // The lobby actor address
        self.lobby_addr
            // We try to send the message to the lobby. -> Waiting it out
            .send(Connect {
                addr: addr.clone(),
                lobby_id: self.room,
                self_id: self.id,
            })
            // We convert to result to ourself
            .into_actor(self)
            // Handle the response
            .then(|res, _self, ctx| {
                match res {
                    Ok(_res) => (),  // if connection success do nothing
                    _ => ctx.stop(), // if failed -> stop the actor/connection
                }
                fut::ready(()) // construct return value
            })
            .wait(ctx); // await the response
    }

    // What to do when the connection is closed
    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.lobby_addr.do_send(Disconnect {
            id: self.id,
            room_id: self.room,
        });
        Running::Stop
    }
}
/// Handle Basic Websocket Message
impl Handler<WsMessage> for WsConn {
    type Result = ();
    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl WsConn {
    /// Spawn heartbeat job every `interval` seconds + disconnect if didnt receive heartbeat for `timeout` seconds
    pub fn spawn_heartbeat_job(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::new(10, 0), |act, _ctx| {
            if Instant::now().duration_since(act.hb).as_secs() > 60 * 2 {
                // heartbeat is dead
                println!("Websocket heartbeat is dead -> Disconnecting");
                act.lobby_addr.do_send(Disconnect {
                    id: act.id,
                    room_id: act.room,
                });
                _ctx.stop();
                return;
            }

            _ctx.ping(&[]);
        });
    }
}

/// Handles the websocket messages received from the client
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(Text(s)) => self
                .lobby_addr
                .try_send(ClientActorMessage {
                    id: self.id,
                    msg: s.to_string(),
                    room_id: self.room,
                })
                .map_or_else(
                    |err| {
                        println!("Error sending message to lobby: {}", err);
                        return ();
                    },
                    |x| (),
                ),
            Err(e) => panic!("{:?}", e),
        }
    }
}

use actix::{
    prelude::{Message, Recipient},
    Actor, Addr, Handler,
};
use uuid::Uuid;

//WsConn responds to this to pipe it through to the actual client
#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

//WsConn sends this to the lobby to say "put me in please"
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect<A>
where
    A: Actor + Handler<WsMessage>,
{
    // pub addr: Recipient<WsMessage>,
    pub addr: Addr<A>,
    pub lobby_id: u128,
    pub self_id: u128,
}

//WsConn sends this to a lobby to say "take me out please"
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub room_id: u128,
    pub id: u128,
}

//client sends this to the lobby for the lobby to echo out.
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub id: u128,
    pub msg: String,
    pub room_id: u128,
}

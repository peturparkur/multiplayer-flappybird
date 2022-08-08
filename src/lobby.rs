// crate::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use actix::{
    dev::ToEnvelope,
    prelude::{Actor, Context, Handler, Recipient},
    Addr, Message,
};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

// mod messages;
use crate::messages::*;

pub struct Lobby<A>
where
    A: Actor + Handler<WsMessage>,
    <A as Actor>::Context: ToEnvelope<A, WsMessage>,
{
    sessions: HashMap<u128, Addr<A>>, // user_id -> ws_connection(user)
    rooms: HashMap<u128, HashSet<u128>>, // room_id -> set of user_ids
}

impl<A> Default for Lobby<A>
where
    A: Actor + Handler<WsMessage>,
    <A as Actor>::Context: ToEnvelope<A, WsMessage>,
{
    fn default() -> Self {
        let lobby = Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::from([(0, HashSet::new())]),
        };
        // lobby.sessions.iter().map(|(k, v)| v.send(msg))
        lobby
    }
}

/// Implement Lobby as Aactor interface
impl<A> Actor for Lobby<A>
where
    A: Actor + Handler<WsMessage>,
    <A as Actor>::Context: ToEnvelope<A, WsMessage>,
{
    type Context = Context<Self>;
}

/// Implement specific functionality for Lobby
impl<A> Lobby<A>
where
    A: Actor + Handler<WsMessage>,
    <A as Actor>::Context: ToEnvelope<A, WsMessage>,
{
    /// Send WsMessage to actor with user_id `id_to`
    fn send_message(&self, message: &str, id_to: &u128) -> Result<(), String> {
        // Try to find the actor with the given user_id
        if let Some(addr) = self.sessions.get(id_to) {
            return addr
                .try_send(WsMessage(message.to_string()))
                .map_err(|err| err.to_string()); // if found we send it the message
                                                 // Ok(())
        } else {
            Err(String::from("User not found"))
        }
    }
}

/// Handler for Disconnect message.
impl<A> Handler<Disconnect> for Lobby<A>
where
    A: Actor + Handler<WsMessage>,
    <A as Actor>::Context: ToEnvelope<A, WsMessage>,
{
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.id).is_some() {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != msg.id)
                .for_each(|user_id| {
                    self.send_message(&format!("{} disconnected.", &msg.id), user_id)
                        .unwrap_or(())
                });
            if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
                if lobby.len() > 1 {
                    lobby.remove(&msg.id);
                } else {
                    //only one in the lobby, remove it entirely
                    self.rooms.remove(&msg.room_id);
                }
            }
        }
    }
}

/// Handling connection message
impl<A> Handler<Connect<A>> for Lobby<A>
where
    A: Actor + Handler<WsMessage>,
    <A as Actor>::Context: ToEnvelope<A, WsMessage>,
{
    type Result = ();

    fn handle(&mut self, msg: Connect<A>, _: &mut Context<Self>) -> Self::Result {
        // create a room if necessary, and then add the id to it
        self.rooms
            .entry(msg.lobby_id)
            .or_insert_with(HashSet::new)
            .insert(msg.self_id);

        // send to everyone in the room that new uuid just joined
        self.rooms
            .get(&msg.lobby_id)
            .unwrap()
            .iter()
            .filter(|conn_id| *conn_id.to_owned() != msg.self_id)
            .for_each(|conn_id| {
                self.send_message(&format!("{} just joined!", msg.self_id), conn_id)
                    .unwrap_or(())
            });

        // store the address
        self.sessions.insert(msg.self_id, msg.addr);

        // send self your new uuid
        self.send_message(&format!("your id is {}", msg.self_id), &msg.self_id)
            .unwrap_or(());
    }
}

/// Handle broadcast message
impl<A> Handler<ClientActorMessage> for Lobby<A>
where
    A: Actor + Handler<WsMessage>,
    <A as Actor>::Context: ToEnvelope<A, WsMessage>,
{
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _: &mut Context<Self>) -> Self::Result {
        if msg.msg.starts_with("\\w") {
            if let Some(id_to) = msg.msg.split(' ').collect::<Vec<&str>>().get(1) {
                if let Ok(id_to) = id_to.parse::<u128>() {
                    self.send_message(
                        &msg.msg
                            .split(' ')
                            .skip(2)
                            .fold(String::from(format!("Whisper[{}]", &msg.id)), |_s, x| {
                                _s + " " + x
                            }),
                        &id_to,
                    )
                    .unwrap_or(());
                }
            }
        } else {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .for_each(|client| self.send_message(&msg.msg, client).unwrap_or(()));
        }
    }
}

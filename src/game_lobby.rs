// crate::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use actix::{
    dev::ToEnvelope,
    prelude::{Actor, Context, Handler, Recipient},
    Addr, AsyncContext, Message,
};
use actix_web_actors::ws;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};
use uuid::Uuid;

// mod messages;
use crate::game::game::Game;
use crate::{game::player::Player, messages::*};

pub struct Lobby<A>
where
    A: Actor + Handler<WsMessage>,
    <A as Actor>::Context: ToEnvelope<A, WsMessage>,
{
    pub sessions: HashMap<u128, Addr<A>>, // user_id -> ws_connection(user)
    pub rooms: HashMap<u128, (HashSet<u128>, Game)>, // room_id -> (set of user_ids, game)
}

impl<A> Default for Lobby<A>
where
    A: Actor + Handler<WsMessage>,
    <A as Actor>::Context: ToEnvelope<A, WsMessage>,
{
    fn default() -> Self {
        let Lobby = Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::from([(0, (HashSet::new(), Game::new()))]),
        };
        // Lobby.sessions.iter().map(|(k, v)| v.send(msg))
        Lobby
    }
}

/// Implement Lobby as Aactor interface
impl<A> Actor for Lobby<A>
where
    A: Actor + Handler<WsMessage>,
    <A as Actor>::Context: ToEnvelope<A, WsMessage>,
{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.spawn_game_update_job(ctx);
    }
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
                .0
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != msg.id)
                .for_each(|user_id| {
                    self.send_message(&format!("{} disconnected.", &msg.id), user_id)
                        .unwrap_or(())
                });
            if let Some((Lobby, game)) = self.rooms.get_mut(&msg.room_id) {
                if Lobby.len() > 1 {
                    Lobby.remove(&msg.id);
                } else {
                    //only one in the Lobby, remove it entirely
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
            .or_insert_with(|| (HashSet::new(), Game::new()))
            .0
            .insert(msg.self_id);

        // send to everyone in the room that new uuid just joined
        self.rooms
            .get(&msg.lobby_id)
            .unwrap()
            .0
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
            return ()
        }
        if msg.msg.starts_with("!") {
            // this means it's a command
            let _msg = msg.msg[1..].to_string(); // convert to slice -> select -> convert to string
            if _msg.starts_with("ready") {
                // start the game
                if let Some((_user_set, _game)) = self.rooms.get_mut(&msg.room_id) {
                    // room.1.start();
                    let _r = _game.ready(msg.id);
                    self.send_message(format!("ready: {}", _r).as_str(), &msg.id);
                }
            }
            return ()
        }
        // else {
        //     self.rooms
        //         .get(&msg.room_id)
        //         .unwrap()
        //         .0
        //         .iter()
        //         .for_each(|client| self.send_message(&msg.msg, client).unwrap_or(()));
        // }

        let _input: bool = match msg.msg.parse() {
            Ok(x) => x,
            Err(err) => {
                println!("Input Can't be parsed: {}", err);
                return ()
            },
        };
        let mut _game = &mut self.rooms.get_mut(&msg.room_id).unwrap().1;
        let r = match _game.set_input(msg.id, _input) {
            Some(_) => {
                // println!("Changed input for player[{}]", msg.id);
                msg.id
            }
            None => {
                println!("found no player with id[{}]", msg.id);
                let u = _game.add_player(Player::new(msg.id));
                // _game.start(); // get fucked! -> TODO: Have message to star game!
                println!("Game State: {:?}", _game);
                self.rooms
                    .get(&msg.room_id)
                    .unwrap()
                    .0
                    .iter()
                    .for_each(|x| {
                        self.send_message(
                            &format!(
                                "new Bird with id [{}]; joined the game with player_id[{}]!",
                                u, msg.id
                            ),
                            x,
                        )
                        .unwrap_or(())
                    });
                u
            }
        };
    }
}

trait Client: Actor + Handler<WsMessage> {}

impl<A> Lobby<A>
where
    A: Actor + Handler<WsMessage>,
    <A as Actor>::Context: ToEnvelope<A, WsMessage>,
{
    pub fn spawn_game_update_job(&self, ctx: &mut Context<Self>) {
        let _basis = 1_000_000_000;
        let _frames = 60;
        let _frame_rate = 1f32 / _frames as f32;
        // let _frame_rate = 33_333_334;
        // let _frame_rate = 16_666_667;
        println!("DEBUG: Spawn game job with interval: {} & dt: {}", _basis / _frames, _frame_rate);
        ctx.run_interval(Duration::new(0, _basis / _frames), move |act, _ctx| {
            // println!(
            //     "T[@{}] updating {} games",
            //     std::time::SystemTime::now()
            //         .duration_since(std::time::UNIX_EPOCH)
            //         .unwrap()
            //         .as_secs(),
            //     act.rooms.len()
            // );
            act.rooms.values_mut().for_each(|(_, game)| {
                game.update(_frame_rate);
            });
            act.rooms.values().for_each(|(users, _game)| {
                users.iter().for_each(|user| {
                    act.send_message(
                        &format!("{}", serde_json::to_string_pretty(_game).unwrap()),
                        user,
                    );
                });
            });
        });
    }
}

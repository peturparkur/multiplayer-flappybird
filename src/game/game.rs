use std::collections::{HashMap, HashSet};
use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};

use super::objects::traits::Position;
use super::{
    objects::{bird::Bird, traits::GameObject, traits::IGameObject, wall::Wall, collision::collision_circle_rectange},
    player::Player,
};
use itertools::Itertools;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct Game {
    pub running: bool,
    players: HashMap<u128, Player>,
    // birds: Vec<Bird>,
    pub birds: HashMap<u128, Bird>,
    pub walls: Vec<Wall>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            running: false,
            players: HashMap::new(),
            // birds: Vec::new(),
            birds: HashMap::new(),
            walls: Vec::new(),
        }
    }
    pub fn add_player(&mut self, player: Player) -> u128 {
        self.players.insert(player.id, player);
        let bird = Bird::new([0.2f32, 0.5f32], [0f32, 0f32], 0.025);
        self.birds.insert(player.id, bird.clone());
        // self.birds.push(bird);
        return bird.id();
    }
    pub fn update(&mut self, dt: f32) {
        // Don't do shit if not running
        if !self.running {
            return;
        }

        for object in self.birds.values_mut() {
            object.update(dt);
        }
        for object in self.walls.iter_mut() {
            object.update(dt);
        }

        self.birds.values_mut().for_each(|bird| {
            for w in self.walls.iter() {
                if collision_circle_rectange(bird.clone(), w.clone()) {
                    println!("collision between BIRB[{:?}] and WALL[{:?}]", &bird, &w);
                    bird.set_active(false);
                    break;
                }
            }
        });//.collect::<Vec<_>>();

        self.walls.retain(|x| x.position()[0] > -0.2); // keep elements where this is true

        if self.walls.len() < 2 {
            let y = Uniform::new(0.0f32, 1.0f32).sample(&mut rand::thread_rng());
            let x = Uniform::new(1.2f32, 1.5f32).sample(&mut rand::thread_rng());
            let wall = Wall::new([x, y], [-0.5f32, 0f32], 0.05, 0.2);
            self.walls.push(wall);
        }
    }

    fn check_start(&self) -> bool {
        if self.players.len() > 0 {
            if self.players.values().all(|x| x.ready) {
                return true;
            }
        }
        return false;
    }

    pub fn ready(&mut self, id: u128) -> bool {
        let r = self.players.get_mut(&id).map(|x| (*x).ready = true).is_some(); // return if there was such a player or not

        if self.check_start() {
            self.start();
        }        

        return r;
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn set_input(&mut self, id: u128, input: bool) -> Option<()> {
        // let player = self.players.iter_mut().find(|player| player.id() == player_id).unwrap();
        // player.set_input(input);
        return self.birds.get_mut(&id).map(|b| b.set_input(input)); // try to find bird -> if found set input
                                                                    // bird.set_input(input);
    }
}

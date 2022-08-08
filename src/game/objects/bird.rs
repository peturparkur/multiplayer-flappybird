use crate::game::objects::traits::*;
use serde::{self, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Bird {
    id: u128,
    position: [f32; 2],
    velocity: [f32; 2],
    radius: f32, // size of visual object

    jump: bool, // true if next turn we jump
    active: bool, // is the player dead?
}

impl Bird {
    pub fn new(pos: [f32; 2], vel: [f32; 2], radius: f32) -> Self {
        Bird {
            id: Uuid::new_v4().as_u128(),
            position: pos.clone(),
            velocity: vel.clone(),
            radius: radius.clone(),
            jump: false,
            active: true,
        }
    }
    pub fn set_input(&mut self, input: bool) {
        self.jump = input;
    }

    pub fn set_active(&mut self, _active: bool) -> bool {
        self.active = _active;
        self.active
    }
}

impl IGameObject for Bird {
    fn id(&self) -> u128 {
        self.id
    }
    fn update(&mut self, dt: f32) -> &Self {
        if !self.active {
            return self;
        }

        if self.jump {
            self.velocity[1] += -self.velocity()[1].min(0f32) + 30.0 * self.gravity() * dt;
            self.jump = false;
        } else {
            self.velocity[1] -= self.gravity() * dt;
        }
        self.velocity[1] = self.velocity[1].max(-1f32).min(60.0 * self.gravity() * dt);
        let new_pos: [f32; 2] = self
            .position()
            .iter()
            .zip(self.velocity().iter())
            .map(|(pos, vel)| pos.clone() + vel.clone() * dt)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        self.set_position(new_pos);
        self
    }
}
impl HasGravity<f32> for Bird {
    fn gravity(&self) -> f32 {
        1f32
    }
}
impl Position<f32> for Bird {
    fn position(&self) -> [f32; 2] {
        [self.position[0] as f32, self.position[1] as f32]
    }
    fn set_position(&mut self, position: [f32; 2]) -> [f32; 2] {
        self.position[0] = position[0] as f32;
        self.position[1] = position[1] as f32;
        self.position()
    }
}
impl Velocity<f32> for Bird {
    fn velocity(&self) -> [f32; 2] {
        [self.velocity[0], self.velocity[1]]
    }
    fn set_velocity(&mut self, velocity: [f32; 2]) -> [f32; 2] {
        self.velocity[0] = velocity[0];
        self.velocity[1] = velocity[1];
        self.velocity()
    }
}

impl Circle<f32> for Bird {
    fn center(&self) -> [f32; 2] {
        [0f32, 0f32]
    }
    fn set_center(&mut self, center: [f32; 2]) -> [f32; 2] {
        // self.position[0] = center[0];
        // self.position[1] = center[1];
        self.center()
    }
    fn radius(&self) -> f32 {
        self.radius
    }
    fn set_radius(&mut self, radius: f32) -> f32 {
        self.radius = radius;
        self.radius
    }
}

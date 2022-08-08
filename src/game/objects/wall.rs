use crate::game::objects::traits::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Wall {
    id: u128,
    position: [f32; 2],
    velocity: [f32; 2],
    width: f32,
    height: f32,
    // radius: f32, // size of visual object

    // jump: bool, // true if next turn we jump
}

impl Wall {
    pub fn new(pos: [f32; 2], vel: [f32; 2], _width: f32, _height: f32) -> Self {
        Wall {
            id: Uuid::new_v4().as_u128(),
            position: pos.clone(),
            velocity: vel.clone(),
            width: _width,
            height: _height,
        }
    }
}


impl IGameObject for Wall {
    fn id(&self) -> u128 {
        self.id
    }
    fn update(&mut self, dt: f32) -> &Self {
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
// impl HasGravity<f32> for Wall {
//     fn gravity(&self) -> f32 {
//         9.81
//     }
// }
impl Position<f32> for Wall {
    fn position(&self) -> [f32; 2] {
        [self.position[0] as f32, self.position[1] as f32]
    }
    fn set_position(&mut self, position: [f32; 2]) -> [f32; 2] {
        self.position[0] = position[0] as f32;
        self.position[1] = position[1] as f32;
        self.position()
    }
}
impl Velocity<f32> for Wall {
    fn velocity(&self) -> [f32; 2] {
        [self.velocity[0], self.velocity[1]]
    }
    fn set_velocity(&mut self, velocity: [f32; 2]) -> [f32; 2] {
        self.velocity[0] = velocity[0];
        self.velocity[1] = velocity[1];
        self.velocity()
    }
}

impl Rectange<f32> for Wall {
    fn center(&self) -> [f32; 2] {
        [0f32, 0f32]
    }
    fn set_center(&mut self, center: [f32; 2]) -> [f32; 2] {
        // self.position[0] = center[0];
        // self.position[1] = center[1];
        self.center()
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn set_width(&mut self, width: f32) -> f32 {
        self.width = width;
        self.width()
    }

    fn height(&self) -> f32 {
        self.height
    }

    fn set_height(&mut self, height: f32) -> f32 {
        self.height = height;
        self.height()
    }
}

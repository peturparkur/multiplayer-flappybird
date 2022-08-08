use std::ops::*;

pub trait Number: Clone + Copy + Default + PartialEq + PartialOrd + Neg<Output = Self> + Add<Output = Self> + Mul<Output = Self> + Sub<Output = Self> + Div<Output = Self> {}
impl Number for f32 {}

pub trait IObject<T: Number>: Position<T> + Velocity<T> + Clone{}

pub trait Velocity<T>
where
    T: Number,
{
    fn velocity(&self) -> [T; 2];
    fn set_velocity(&mut self, velocity: [T; 2]) -> [T; 2];
}

pub trait Position<T>
where
    T: Number,
{
    fn position(&self) -> [T; 2];
    fn set_position(&mut self, position: [T; 2]) -> [T; 2];
}

pub trait IGameObject {
    fn id(&self) -> u128;
    fn update(&mut self, dt: f32) -> &Self;
}

pub trait HasGravity<T>
where
    T: Number,
{
    fn gravity(&self) -> T;
    // fn set_gravity(&mut self, gravity: T) -> T;
}

pub trait Circle<T>: Position<T>
where
    T: Number,
{
    /// center is described as offset from position
    fn center(&self) -> [T; 2];
    fn set_center(&mut self, center: [T; 2]) -> [T; 2];
    fn radius(&self) -> T;
    fn set_radius(&mut self, radius: T) -> T;
}

pub trait Rectange<T>: Position<T>
where
    T: Number,
{
    /// center is described as offset from position
    fn center(&self) -> [T; 2];
    fn set_center(&mut self, center: [T; 2]) -> [T; 2];
    fn width(&self) -> T;
    fn set_width(&mut self, width: T) -> T;
    fn height(&self) -> T;
    fn set_height(&mut self, height: T) -> T;
}

pub struct GameObject {
    id: u128,
}
impl IGameObject for GameObject {
    fn id(&self) -> u128 {
        self.id
    }
    fn update(&mut self, dt: f32) -> &Self {
        self
    }
}

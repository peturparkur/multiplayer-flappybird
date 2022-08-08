use itertools::Itertools;

use super::traits::{Number, Rectange, Circle};



pub fn collision_circle<T: Number, C: Circle<T>>(a: C, b: C) -> bool 
{
    let dist = a.position().iter().zip(b.position().iter()).map(|(a, b)| {
        a.clone() - b.clone()
    }).fold(T::default(), |_sum, x| _sum + x*x);
    dist < a.radius() + b.radius()
    // dist < a.radius() + b.radius()
}

// to get absolute value -> 
fn abs<T>(x: T) -> T 
where
    T: Number
{
    if x < T::default() {
        return -x
    }
    return x
}

pub fn collision_circle_rectange<T: Number, C: Circle<T>, R: Rectange<T>>(a: C, b: R) -> bool 
where
    T: std::ops::Mul<f32, Output = T>
{

    // 2D offset
    let offset = a.position().iter().zip(b.position()).map(|(a, b)| {
        a.clone() - b.clone()
    }).collect_vec();

    // potential collision on x axis
    if (b.width() * 0.5 + a.radius()) < abs(offset[0]) {
        return false;
    }
    // potential collision on y axis
    if (b.height() * 0.5 + a.radius()) < abs(offset[1]) {
        return false;
    }
    return true;
}
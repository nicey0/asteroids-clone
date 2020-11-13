use super::consts::*;
use super::ship::Ship;

// cosine math
pub fn cos_math(m: f64, rot: f64) -> f64 {
    m * (rot * RADTODEG).cos()
}

// sine math
pub fn sin_math(m: f64, rot: f64) -> f64 {
    m * (rot * RADTODEG).sin()
}

//pub fn look_at(ship: &Ship, x: f64, y: f64) {}

use super::asteroid::Asteroid;
use super::consts::*;
use super::util::*;

// cosine math
pub fn cos_math(m: f64, rot: f64) -> f64 {
    m * (rot * RADTODEG).cos()
}

// sine math
pub fn sin_math(m: f64, rot: f64) -> f64 {
    m * (rot * RADTODEG).sin()
}

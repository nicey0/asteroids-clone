use super::consts::*;

pub fn cos_math(m: f64, rot: f64) -> f64 {
    m * (rot * RADTODEG).cos()
}

pub fn sin_math(m: f64, rot: f64) -> f64 {
    m * (rot * RADTODEG).sin()
}

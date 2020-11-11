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

// collision math
fn inside_square(x: f64, y: f64, ast: &Asteroid) -> bool {
    let sx = ast.get_x();
    let sy = ast.get_y();
    let sw = ast.get_w();
    sx < x && x < sx + sw && sy < y && y < sy + sw
}

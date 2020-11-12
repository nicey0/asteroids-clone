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
fn inside_circle(x: f64, y: f64, ast: &Asteroid) -> bool {
    let sx = ast.get_x();
    let sy = ast.get_y();
    let sr = ast.get_r();
    if ((x - sx).powf(2.0) + (y - sy).powf(2.0)).powf(0.5) <= sr {
        return true;
    }
    false
}

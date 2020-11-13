use super::consts::*;
use super::ship::Ship;
use super::util::*;

// cosine math
pub fn cos_math(m: f64, rot: f64) -> f64 {
    m * (rot * RADTODEG).cos()
}

// sine math
pub fn sin_math(m: f64, rot: f64) -> f64 {
    m * (rot * RADTODEG).sin()
}

fn normalize_angle(a: f64) -> f64 {
    return if a < 0.0 { a + 360.0 } else { a };
}

pub fn look_at(ship: &mut Ship, x: f64, y: f64) {
    let dx = ship.get_x() - x;
    let dy = ship.get_y() - y;
    let a = (normalize_angle(dy.atan2(dx)) - ship.get_rot()) % 360.0;
    ship.set_rot(a / a.abs() * 2.0);
}

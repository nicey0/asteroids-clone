use super::consts::*;
use super::math::*;

#[derive(Clone, Copy)]
pub struct Bullet {
    x: f64,
    y: f64,
    xspd: f64,
    yspd: f64,
    rot: f64,
    len: f64,
}

impl Bullet {
    pub fn new(ship: &Ship) -> Self {
        Self {
            len: BULLEN,
            x: ship.x,
            y: ship.y,
            xspd: ship.xspd + 5.0,
            yspd: ship.yspd + 5.0,
            rot: ship.rot,
        }
    }

    pub fn tick(&mut self) -> bool {
        self.x += self.xspd;
        self.y += self.yspd;
        // check out of bounds
        !(self.x < -(PADDING as f64)
            || self.x > DIM as f64 + PADDING as f64
            || self.y < -(PADDING as f64)
            || self.y > DIM as f64 + PADDING as f64)
    }
}

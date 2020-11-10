use super::consts::*;

#[derive(Debug)]
pub struct Ship {
    x: f64,
    y: f64,
    xspd: f64,
    yspd: f64,
    rot: f64,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            xspd: 0.0,
            yspd: 0.0,
            rot: 0.0,
        }
    }

    pub fn rotate(&mut self, deg: f64) {
        self.rot += deg;
    }

    pub fn accelerate(&mut self, acc: f64) {
        self.xspd += cos_math(acc, self.rot);
        self.yspd += sin_math(acc, self.rot);
    }

    pub fn tick(&mut self) {
        self.x += self.xspd;
        self.y += self.yspd;
        if self.x < -(PADDING as f64) {
            self.x = (DIM + PADDING - 1) as f64;
        } else if self.x >= (DIM + PADDING) as f64 {
            self.x = 0.0;
        }
        if self.y < -(PADDING as f64) {
            self.y = (DIM + PADDING - 1) as f64;
        } else if self.y >= (DIM + PADDING) as f64 {
            self.y = 0.0;
        }
    }
}

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

fn cos_math(m: f64, rot: f64) -> f64 {
    m * (rot * RADTODEG).cos()
}

fn sin_math(m: f64, rot: f64) -> f64 {
    m * (rot * RADTODEG).sin()
}

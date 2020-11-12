use super::consts::*;
use super::math::*;
use super::util::*;

#[derive(Debug)]
pub struct Ship {
    x: f64,
    y: f64,
    xspd: f64,
    yspd: f64,
    rot: f64,
    size: f64,
    front: f64,
}

impl Ship {
    pub fn new(size: f64) -> Self {
        Self {
            x: DIM as f64 / 2.0,
            y: DIM as f64 / 2.0,
            xspd: 0.0,
            yspd: 0.0,
            rot: 0.0,
            size: size,
            front: size * 3.0,
        }
    }

    pub fn rotate(&mut self, deg: f64) {
        self.rot += deg;
    }

    pub fn accelerate(&mut self, acc: f64) {
        self.xspd += cos_math(acc, self.rot);
        self.yspd += sin_math(acc, self.rot);
        if self.xspd >= MAXSPEED {
            self.xspd = MAXSPEED;
        } else if self.xspd <= -MAXSPEED {
            self.xspd = -MAXSPEED;
        }
        if self.yspd >= MAXSPEED {
            self.yspd = MAXSPEED;
        } else if self.yspd <= -MAXSPEED {
            self.yspd = -MAXSPEED;
        }
    }

    pub fn get_points(&self) -> [APoint; 4] {
        [
            [
                self.x + cos_math(self.size, self.rot - 50.0),
                self.y + sin_math(self.size, self.rot - 50.0),
            ],
            [self.x, self.y],
            [
                self.x + cos_math(self.size, self.rot + 50.0),
                self.y + sin_math(self.size, self.rot + 50.0),
            ],
            [
                self.x + cos_math(self.front, self.rot),
                self.y + sin_math(self.front, self.rot),
            ],
        ]
    }

    pub fn tick(&mut self) {
        self.x += self.xspd;
        self.y += self.yspd;
        if self.x < -(PADDING as f64) {
            self.x = (DIM + PADDING - 1) as f64;
        } else if self.x >= (DIM + PADDING) as f64 {
            self.x = 0.1 - PADDING as f64;
        }
        if self.y < -(PADDING as f64) {
            self.y = (DIM + PADDING - 1) as f64;
        } else if self.y >= (DIM + PADDING) as f64 {
            self.y = 0.1 - PADDING as f64;
        }
    }

    pub fn get_size(&self) -> f64 {
        self.size
    }

    pub fn get_speed(&self) -> (f64, f64) {
        (self.xspd, self.yspd)
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Bullet {
    x: f64,
    y: f64,
    xspd: f64,
    yspd: f64,
    rad: f64,
}

impl Bullet {
    pub fn new(ship: &Ship) -> Self {
        Self {
            x: ship.x,
            y: ship.y,
            xspd: ship.xspd + 5.0,
            yspd: ship.yspd + 5.0,
            rad: BULLEN,
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

    pub fn get_rad(&self) -> f64 {
        self.rad
    }
}

impl Coord for Ship {
    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }
}

impl Coord for Bullet {
    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }
}

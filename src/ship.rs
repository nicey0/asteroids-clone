use super::conf::*;
use super::math::*;
use super::util::*;

#[derive(Debug)]
pub struct Ship {
    x: f64,
    y: f64,
    xspd: f64,
    yspd: f64,
    rot: f64,
    cdeg: f64,
    size: f64,
    front: f64,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            x: DIM as f64 / 2.0,
            y: DIM as f64 / 2.0,
            xspd: 0.0,
            yspd: 0.0,
            rot: 0.0,
            cdeg: 0.0,
            size: SHIPSIZE,
            front: SHIPSIZE * 3.0,
        }
    }

    pub fn rotate(&mut self, deg: f64) {
        self.cdeg += deg;
        if self.cdeg > 5.0 {
            self.cdeg = 5.0;
        } else if self.cdeg < -5.0 {
            self.cdeg = -5.0;
        }
    }

    pub fn accelerate(&mut self, acc: f64) {
        self.xspd += cos_math(acc, self.rot as f64);
        self.yspd += sin_math(acc, self.rot as f64);
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

    pub fn tick(&mut self) {
        self.x += self.xspd;
        self.y += self.yspd;
        self.rot += self.cdeg;
        if self.cdeg > 0.0 && self.cdeg <= 5.0 {
            self.cdeg -= 0.8;
        } else if self.cdeg < 0.0 && self.cdeg >= -5.0 {
            self.cdeg += 0.8;
        }
        if -0.8 < self.cdeg && self.cdeg < 0.8 {
            self.cdeg = 0.0;
        }
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

    pub fn get_points(&self) -> [APoint; 4] {
        [
            [
                self.x + cos_math(self.size, (self.rot - 90.0) as f64),
                self.y + sin_math(self.size, (self.rot - 90.0) as f64),
            ],
            [
                self.x + cos_math(self.size, (self.rot - 180.0) as f64),
                self.y + sin_math(self.size, (self.rot - 180.0) as f64),
            ],
            [
                self.x + cos_math(self.size, (self.rot + 90.0) as f64),
                self.y + sin_math(self.size, (self.rot + 90.0) as f64),
            ],
            [
                self.x + cos_math(self.front, self.rot),
                self.y + sin_math(self.front, self.rot),
            ],
        ]
    }

    pub fn get_size(&self) -> f64 {
        self.size
    }

    pub fn get_speed(&self) -> (f64, f64) {
        (self.xspd, self.yspd)
    }

    pub fn get_rot(&self) -> f64 {
        self.rot
    }

    pub fn add_rot(&mut self, rot: f64) {
        self.rot += rot;
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Bullet {
    x: f64,
    y: f64,
    xspd: f64,
    yspd: f64,
    r: f64,
}

impl Bullet {
    pub fn new(ship: &Ship) -> Self {
        Self {
            x: ship.x + cos_math(ship.size * 3.0, ship.rot as f64),
            y: ship.y + sin_math(ship.size * 3.0, ship.rot as f64),
            xspd: ship.xspd + cos_math(BULSPD, ship.rot as f64),
            yspd: ship.yspd + sin_math(BULSPD, ship.rot as f64),
            r: BULRAD,
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

    pub fn get_r(&self) -> f64 {
        self.r
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

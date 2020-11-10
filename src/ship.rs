use super::consts::*;
use super::math::*;

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
            x: DIM as f64 / 2.0,
            y: DIM as f64 / 2.0,
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

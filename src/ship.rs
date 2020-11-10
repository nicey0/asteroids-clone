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
        self.xspd += self.cos_math(acc);
        self.yspd += self.sin_math(acc);
    }

    pub fn tick(&mut self) {
        self.x += self.xspd;
        self.y += self.yspd;
        if self.x < 0.0 {
            self.x = (WIDTH + PADDING - 1) as f64;
        } else if self.x >= (WIDTH + PADDING) as f64 {
            self.x = 0.0;
        }
        if self.y < 0.0 {
            self.y = (HEIGHT + PADDING - 1) as f64;
        } else if self.y >= (HEIGHT + PADDING) as f64 {
            self.y = 0.0;
        }
    }

    fn cos_math(&mut self, m: f64) -> f64 {
        m * (self.rot * RADTODEG).cos()
    }

    fn sin_math(&mut self, m: f64) -> f64 {
        m * (self.rot * RADTODEG).sin()
    }
}

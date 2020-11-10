use std::f64::consts::PI;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;
const RADTODEG: f64 = 2.0 * (PI / 360.0);

#[derive(Debug)]
struct Ship {
    x: f64,
    y: f64,
    xspd: f64,
    yspd: f64,
    rot: f64,
}

impl Ship {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            xspd: 0.0,
            yspd: 0.0,
            rot: 0.0,
        }
    }

    fn cos_math(&mut self, m: f64) -> f64 {
        m * (self.rot * RADTODEG).cos()
    }

    fn sin_math(&mut self, m: f64) -> f64 {
        m * (self.rot * RADTODEG).sin()
    }

    fn rotate(&mut self, deg: f64) {
        self.rot += deg;
    }

    fn accelerate(&mut self, acc: f64) {
        self.xspd += self.cos_math(acc);
        self.yspd += self.sin_math(acc);
    }

    fn tick(&mut self) {
        self.x += self.xspd;
        self.y += self.yspd;
    }
}

fn main() {
    let mut ship = Ship::new();
}

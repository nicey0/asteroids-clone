use rand::{thread_rng, Rng};

use super::util::*;

pub type Explosion = Vec<Particle>;

#[derive(Clone)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub xspd: f64,
    pub yspd: f64,
    timer: u8,
}

impl Particle {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            xspd: thread_rng().gen_range(-1.0, 1.0),
            yspd: thread_rng().gen_range(-1.0, 1.0),
            timer: 10,
        }
    }

    pub fn tick(&mut self) -> bool {
        return if self.timer == 0 {
            false
        } else {
            self.x += self.xspd;
            self.y += self.yspd;
            self.timer -= 1;
            true
        }
    }
}


impl Coord for Particle {
    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }
}

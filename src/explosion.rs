use rand::distributions::Distribution;

use super::randstuff::Ranges;
use super::util::*;

pub type Particles = Vec<Particle>;

#[derive(Clone, Copy)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub xspd: f64,
    pub yspd: f64,
    timer: u8,
}

impl Particle {
    pub fn new(x: f64, y: f64, rr: &mut Ranges) -> Self {
        Self {
            x,
            y,
            xspd: rr.p_speed.sample(&mut rr.rng),
            yspd: rr.p_speed.sample(&mut rr.rng),
            timer: 20,
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
        };
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

use rand::{thread_rng, Rng};

use super::util::*;

#[derive(Clone, Copy)]
pub struct Particle {
    x: f64,
    y: f64,
    xspd: f64,
    yspd: f64,
    timer: u8,
}

impl Particle {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            xspd: thread_rng().gen_range(-2.0, 2.0),
            yspd: thread_rng().gen_range(-2.0, 2.0),
            timer: 15,
        }
    }
}

pub struct Explosion {
    x: f64,
    y: f64,
    parts: [Particle; 15]
}

impl Explosion {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            parts: [Particle::new(x, y); 15],
        }
    }

    pub fn get_parts(&self) -> [Particle; 15] {
        self.parts
    }
}

impl Coord for Explosion {
    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }
}

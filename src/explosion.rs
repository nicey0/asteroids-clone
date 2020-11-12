use rand::{thread_rng, Rng};

use super::util::*;

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
}

pub struct Explosion {
    x: f64,
    y: f64,
    pub parts: Vec<Particle>,
}

impl Explosion {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            parts: Self::gen_parts(x, y),
        }
    }

    fn gen_parts(x: f64, y: f64) -> Vec<Particle> {
        let mut v: Vec<Particle> = Vec::new();
        for _ in 0..15 {
            v.push(Particle::new(x, y));
        }
        v
    }

    pub fn get_parts(&self) -> &Vec<Particle> {
        &self.parts
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

impl Coord for Explosion {
    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }
}

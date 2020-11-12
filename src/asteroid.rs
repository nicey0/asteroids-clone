use super::consts::*;
use super::math::*;
use super::util::APoint;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Asteroid {
    x: f64,
    y: f64,
    xspd: f64,
    yspd: f64,
    w: f64,
    points: Vec<APoint>,
}

impl Asteroid {
    pub fn new() -> Self {
        let ((x, xspd), (y, yspd)) = Self::get_random_xy();
        Self {
            x,
            y,
            xspd,
            yspd,
            w: 30.0,
            points: Self::gen_points(x, y, 30.0),
        }
    }

    fn gen_points(x: f64, y: f64, w: f64) -> Vec<APoint> {
        let mut v = Vec::new();
        for i in 0..11 {
            let d = thread_rng().gen_range(w / 2.0, w);
            let angle = (360.0 / 5.0) * i as f64;
            let px = cos_math(d, angle);
            let py = sin_math(d, angle);
            v.push([x + px, y + py]);
        }
        v
    }

    fn get_random_xy() -> ((f64, f64), (f64, f64)) {
        let fh = match thread_rng().gen_range(0, 2) {
            0 => (-(PADDING as f64) + 0.1, random_spd()),
            _ => ((DIM + PADDING) as f64 - 0.1, random_spd() * -1.0),
        };
        let fd = match thread_rng().gen_range(0, 2) {
            0 => (rand_mid(), random_spd()),
            _ => (rand_mid() + DIM as f64 / 2.0 - 0.1, random_spd() * -1.0),
        };
        return match thread_rng().gen_range(0, 2) {
            0 => (fh, fd),
            _ => (fd, fh),
        };
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

    pub fn print(&self) {
        println!(
            "x:    {}\ny:    {}\nxspd: {}\nyspd: {}",
            self.x, self.y, self.xspd, self.yspd
        );
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_w(&self) -> f64 {
        self.w
    }

    pub fn get_points(&self) -> Vec<APoint> {
        let mut v2 = self.points.clone();
        for p in v2.iter_mut() {
            p[0] += self.x;
            p[1] += self.y;
        }
        v2
    }
}

fn get_random_dir(min: f64, max: f64, spdm: f64) -> (f64, f64) {
    let d = thread_rng().gen_range(min, max);
    let dspd = thread_rng().gen_range(0.0, ASTSPD) * spdm;
    (d, dspd)
}

fn rand_mid() -> f64 {
    thread_rng().gen_range(0.0, DIM as f64 / 2.0)
}

fn random_spd() -> f64 {
    thread_rng().gen_range(0.0, ASTSPD)
}

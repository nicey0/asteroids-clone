use super::consts::*;
use super::math::*;
use super::randstuff::*;
use super::util::APoint;

use rand::distributions::Distribution;

#[derive(Debug, Clone)]
pub struct Asteroid {
    x: f64,
    y: f64,
    xspd: f64,
    yspd: f64,
    r: f64,
    points: Vec<APoint>,
}

impl Asteroid {
    pub fn new(rr: &mut Ranges) -> Self {
        let ((x, xspd), (y, yspd)) = Self::get_random_xy(rr);
        let r = rr.ast_r.sample(&mut rr.rng);
        Self {
            x,
            y,
            xspd,
            yspd,
            r,
            points: Self::gen_points(x, y, r, rr),
        }
    }

    fn gen_points(x: f64, y: f64, r: f64, rr: &mut Ranges) -> Vec<APoint> {
        let mut v = Vec::new();
        let edges = rr.ast_edges.sample(&mut rr.rng);
        for i in 0..edges {
            let d = rr.ast_round.sample(&mut rr.rng);
            let angle = (360.0 / edges as f64) * i as f64;
            let px = cos_math(d, angle);
            let py = sin_math(d, angle);
            v.push([x + px, y + py]);
        }
        v
    }

    fn get_random_xy(rr: &mut Ranges) -> ((f64, f64), (f64, f64)) {
        let fh = match rr.zero_one.sample(&mut rr.rng) {
            0 => (-(PADDING as f64) + 0.1, rr.ast_speed.sample(&mut rr.rng)),
            _ => (
                (DIM + PADDING) as f64 - 0.1,
                rr.ast_speed.sample(&mut rr.rng) * -1.0,
            ),
        };
        let fd = match rr.zero_one.sample(&mut rr.rng) {
            0 => (
                rr.dim_half.sample(&mut rr.rng),
                rr.ast_speed.sample(&mut rr.rng),
            ),
            _ => (
                rr.dim_half.sample(&mut rr.rng) + DIM as f64 / 2.0 - 0.1,
                rr.ast_speed.sample(&mut rr.rng) * -1.0,
            ),
        };
        return match rr.zero_one.sample(&mut rr.rng) {
            0 => (fh, fd),
            _ => (fd, fh),
        };
    }

    pub fn tick(&mut self) -> bool {
        self.x += self.xspd;
        self.y += self.yspd;
        // update points
        for p in self.points.iter_mut() {
            p[0] += self.xspd;
            p[1] += self.yspd;
        }
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

    pub fn get_r(&self) -> f64 {
        self.r
    }

    pub fn get_points(&self) -> &Vec<APoint> {
        &self.points
    }
}

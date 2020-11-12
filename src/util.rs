pub enum States {
    Nothing,
    Score,
    GameOver,
}

pub trait Coord {
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;
}

pub type Point = (f64, f64);
pub type APoint = [f64; 2];

pub struct Pressed {
    pub d: bool,
    pub a: bool,
    pub w: bool,
    pub space: bool,
}

impl Pressed {
    pub fn new() -> Self {
        Pressed {
            d: false,
            a: false,
            w: false,
            space: false,
        }
    }
}

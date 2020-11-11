pub trait Coord {
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;
}

pub type Point = (f64, f64);
pub type APoint = [f64; 2];

#[derive(PartialEq)]
pub enum Direction {
    Collinear,
    Clockwise,
    Anticlockwise,
}

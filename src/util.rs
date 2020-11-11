pub trait Coord {
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;
}

pub type Point = (f64, f64);

#[derive(PartialEq)]
pub enum Direction {
    Collinear,
    Clockwise,
    Anticlockwise,
}

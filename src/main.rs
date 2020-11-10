extern crate rand;
mod asteroid;
mod consts;
mod ship;
use asteroid::Asteroid;
use consts::*;
use ship::Ship;

fn main() {
    let ship = Ship::new();
    println!("{} {}", DIM, DIM);
    for _ in 0..5 {
        Asteroid::new().print();
    }
}

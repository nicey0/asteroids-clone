extern crate rand;
mod asteroid;
mod consts;
mod ship;
use asteroid::Asteroid;
use consts::*;
use ship::Ship;
use std::{thread::sleep, time::Duration};

fn main() {
    //let ship = Ship::new();
    println!("{} {}", DIM, DIM);
    let mut asts: [Asteroid; 10] = [Asteroid::new(); 10];
    loop {
        print!("\x1B[2J\x1B[1;1H");
        for ast in asts.iter_mut() {
            if !ast.tick() {
                *ast = Asteroid::new();
            };
            ast.print();
            println!("");
        }
        sleep(Duration::from_millis(100));
    }
}

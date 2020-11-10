use std::{thread::sleep, time::Duration};

mod consts;
use consts::*;

mod asteroid;
mod bullet;
mod math;
mod ship;
use asteroid::Asteroid;
use bullet::Bullet;
use ship::Ship;

fn main() {
    //let ship = Ship::new();
    println!("{} {}", DIM, DIM);
    let mut ship = Ship::new();
    let mut asts: [Asteroid; 10] = [Asteroid::new(); 10];
    let mut buls: Vec<Bullet> = Vec::new();
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

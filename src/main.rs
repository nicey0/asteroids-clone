use std::{thread::sleep, time::Duration};

mod consts;
use consts::*;

mod asteroid;
mod col;
mod math;
mod ship;
mod util;
use asteroid::Asteroid;
use col::*;
use ship::*;

fn main() {
    println!("{} {}", DIM, DIM);
    let mut ship = Ship::new();
    let mut asts: [Asteroid; 10] = [Asteroid::new(); 10];
    let mut buls: Vec<Bullet> = Vec::new();
    loop {
        print!("\x1B[2J\x1B[1;1H");
        // tick ship
        ship.tick();
        // tick bullets
        buls = buls
            .iter_mut()
            .filter_map(|bul| return if !bul.tick() { None } else { Some(*bul) })
            .collect();
        // tick asteroids
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

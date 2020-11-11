use std::{thread::sleep, time::Duration};

use piston_window::*;

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

    let mut window: PistonWindow =
        WindowSettings::new("Asteroids clone much better than meesocks'", [DIM; 2])
            .exit_on_esc(true)
            .resizable(false)
            .decorated(true)
            .build()
            .unwrap();
    while let Some(e) = window.next() {
        print!("\x1B[2J\x1B[1;1H");

        /*
         * Update
         */
        ship.tick();
        buls = buls // tick bullets
            .iter_mut()
            .filter_map(|bul| return if !bul.tick() { None } else { Some(*bul) })
            .collect();
        for ast in asts.iter_mut() {
            // tick asteroids
            if !ast.tick() {
                *ast = Asteroid::new();
            };
        }

        /*
         * Render
         */
        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            //rectangle(
            //[1.0, 0.0, 1.0, 1.0],
            //[0.0, 0.0, 100.0, 100.0],
            //c.transform,
            //g,
            //);
            line(
                [1.0, 0.0, 1.0, 1.0],
                1.0,
                [100.0, 100.0, 300.0, 300.0],
                c.transform,
                g,
            );
        });
    }
    loop {}
}

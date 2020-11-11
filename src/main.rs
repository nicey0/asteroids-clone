use std::{thread::sleep, time::Duration};

use piston_window::*;

mod consts;
use consts::*;

mod asteroid;
mod col;
mod math;
mod render;
mod ship;
mod util;
use asteroid::Asteroid;
use math::*;
use render::*;
use ship::*;

fn main() {
    let mut ship = Ship::new();
    let mut asts: Vec<Asteroid> = Vec::new();
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
        update(&mut ship, &mut buls, &mut asts);
        /*
         * Render
         */
        window.draw_2d(&e, |c, g, _| {
            render(&c, g, &mut ship, &mut buls, &mut asts);
        });
    }
}

fn update(ship: &mut Ship, buls: &mut Vec<Bullet>, asts: &mut Vec<Asteroid>) {
    tick_stuff(ship, buls, asts);
}

fn tick_stuff(ship: &mut Ship, buls: &mut Vec<Bullet>, asts: &mut Vec<Asteroid>) {
    ship.tick();
    *buls = buls
        .iter_mut()
        .filter_map(|bul| return if !bul.tick() { None } else { Some(*bul) })
        .collect();
    for ast in asts.iter_mut() {
        if !ast.tick() {
            *ast = Asteroid::new();
        };
    }
}

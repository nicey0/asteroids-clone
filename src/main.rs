use std::{thread::sleep, time::Duration};

use piston_window::*;

mod consts;
use consts::*;

mod asteroid;
mod col;
mod math;
mod render;
mod ship;
mod update;
mod util;
use asteroid::Asteroid;
use render::*;
use ship::*;
use update::*;
use util::*;

fn main() {
    let mut ship = Ship::new(12.0);
    let mut p = Pressed::new();
    let mut asts: Vec<Asteroid> = Vec::new();
    let mut buls: Vec<Bullet> = Vec::new();
    let mut window: PistonWindow =
        WindowSettings::new("Asteroids clone much better than meesocks'", [DIM; 2])
            .exit_on_esc(true)
            .resizable(false)
            .decorated(true)
            .build()
            .unwrap();
    window.set_ups(20);
    ship.rotate(-45);
    while let Some(e) = window.next() {
        print!("\x1B[2J\x1B[1;1H");
        println!("{:?}", ship.get_speed());
        update(&mut ship, &p, &mut buls, &mut asts);
        window.draw_2d(&e, |c, g, _| {
            render(&c, g, &mut ship, &mut buls, &mut asts);
        });
        if let Some(b) = e.button_args() {
            match b.state {
                ButtonState::Press => {
                    if let Button::Keyboard(k) = b.button {
                        match k {
                            Key::D => p.d = true,
                            Key::A => p.a = true,
                            Key::W => p.w = true,
                            _ => {}
                        }
                    }
                }
                ButtonState::Release => {
                    if let Button::Keyboard(k) = b.button {
                        match k {
                            Key::D => p.d = false,
                            Key::A => p.a = false,
                            Key::W => p.w = false,
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

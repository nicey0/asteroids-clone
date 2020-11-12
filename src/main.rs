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
use col::*;
use math::*;
use render::*;
use ship::*;
use update::*;
use util::*;

fn main() {
    let mut ship = Ship::new(12.0);
    let mut p = Pressed::new();
    let mut asts: Vec<Asteroid> = Vec::new();
    let mut cooldown = 0;
    let mut score = 0;

    for _ in 0..AST_COUNT {
        asts.push(Asteroid::new());
    }
    let mut buls: Vec<Bullet> = Vec::new();
    let mut window: PistonWindow =
        WindowSettings::new("Asteroids clone much better than meesocks'", [DIM; 2])
            .exit_on_esc(true)
            .resizable(false)
            .decorated(true)
            .build()
            .unwrap();
    window.set_ups(30);

    let mut glyphs = window
        .load_font("fonts/FiraSans-Regular.ttf")
        .expect("error loading font!");
    while let Some(e) = window.next() {
        print!("\x1B[2J\x1B[1;1H");
        //println!("{:?}", ship.get_speed());
        match update(&mut ship, &p, &mut buls, &mut asts) {
            States::GameOver => break,
            States::Score => score += 10,
            States::Nothing => {}
        };
        if cooldown > 0 {
            cooldown -= 1;
        }
        window.draw_2d(&e, |c, g, dev| {
            render(&c, g, &mut ship, &mut buls, &mut asts);
            Text::new_color([1.0; 4], FSIZE)
                .draw(
                    &format!("Score: {}", score),
                    &mut glyphs,
                    &c.draw_state,
                    c.transform
                        .trans(PADDING as f64 / 3.0, FSIZE as f64 + (PADDING as f64 / 3.0)),
                    g,
                )
                .expect("error drawing text!");
            glyphs.factory.encoder.flush(dev);
        });
        if let Some(b) = e.button_args() {
            match b.state {
                ButtonState::Press => {
                    if let Button::Keyboard(k) = b.button {
                        match k {
                            Key::D => p.d = true,
                            Key::A => p.a = true,
                            Key::W => p.w = true,
                            Key::Space => {
                                if cooldown == 0 {
                                    buls.push(Bullet::new(&ship));
                                    cooldown = BULCOOLDOWN;
                                }
                            }
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

use piston_window::*;

mod conf;
use conf::*;

mod asteroid;
mod col;
mod consts;
mod explosion;
mod math;
mod randstuff;
mod render;
mod ship;
mod update;
mod util;
use asteroid::*;
use explosion::*;
use randstuff::Ranges;
use render::*;
use ship::*;
use update::*;
use util::*;

fn main() {
    let mut ship = Ship::new();
    let mut p = Pressed::new();
    let mut asts: Vec<Asteroid> = Vec::with_capacity(AST_COUNT);
    let mut parts: Particles = Vec::with_capacity(AST_COUNT);
    let mut cooldown = 0;
    let mut score = 0;
    let mut mx = 0.0;
    let mut my = 0.0;

    let mut rr = Ranges::new();
    for _ in 0..AST_COUNT {
        asts.push(Asteroid::new(&mut rr));
    }
    let mut buls: Vec<Bullet> = Vec::new();
    let mut window: PistonWindow =
        WindowSettings::new("Asteroids clone much better than meesocks'", [DIM; 2])
            .exit_on_esc(true)
            .resizable(false)
            .decorated(true)
            .build()
            .unwrap();
    window.set_ups(60);
    window.set_max_fps(60);

    let mut glyphs = window
        .load_font("fonts/november.ttf")
        .expect("error loading font!");
    while let Some(e) = window.next() {
        //print!("\x1B[2J\x1B[1;1H"); // clear screen
        //println!("{}, {}", mx, my);
        if let Some(_) = e.update_args() {
            match update(&mut ship, &p, &mut buls, &mut asts, &mut parts, &mut rr) {
                States::GameOver => break,
                States::Score => score += 10,
                States::Nothing => {}
            };
            if cooldown > 0 {
                cooldown -= 1;
            }
        } else if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g, dev| {
                render(&c, g, &mut ship, &mut buls, &mut asts, &parts);
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
        } else if let Some(coor) = e.mouse_cursor_args() {
            mx = coor[0];
            my = coor[1];
        } else if let Some(b) = e.button_args() {
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

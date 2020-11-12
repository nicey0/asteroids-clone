use super::asteroid::*;
use super::col::*;
use super::consts::*;
use super::ship::*;
use super::util::*;

pub fn update(
    ship: &mut Ship,
    p: &Pressed,
    buls: &mut Vec<Bullet>,
    asts: &mut Vec<Asteroid>,
) -> States {
    if p.a {
        ship.rotate(-ROTSPEED * 0.8);
    } else if p.d {
        ship.rotate(ROTSPEED * 0.8);
    }
    if p.w {
        ship.accelerate(ACCSPEED / 0.8);
    }
    for ast in asts.iter() {
        if ship_in_asteroid(ship, ast) {
            return States::GameOver;
        }
    }
    for bul in buls.iter_mut() {
        for ast in asts.iter_mut() {
            if inside_circle(bul.get_x(), bul.get_y(), ast) {
                *ast = Asteroid::new();
                return States::Score;
            };
        }
    }
    tick_stuff(ship, buls, asts);
    States::Nothing
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

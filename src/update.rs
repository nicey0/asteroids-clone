use super::asteroid::*;
use super::col::*;
use super::math::*;
use super::ship::*;
use super::util::*;

pub fn update(
    ship: &mut Ship,
    p: &Pressed,
    buls: &mut Vec<Bullet>,
    asts: &mut Vec<Asteroid>,
) -> States {
    if p.a {
        ship.rotate(-1);
    } else if p.d {
        ship.rotate(1);
    }
    for point in &ship.get_points() {
        for ast in asts.iter() {
            if inside_circle(point[0], point[1], ast) {
                return States::GameOver;
            }
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
    if p.w {
        ship.accelerate(0.005);
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

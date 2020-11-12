use super::asteroid::*;
use super::math::*;
use super::ship::*;
use super::util::*;

pub fn update(ship: &mut Ship, p: &Pressed, buls: &mut Vec<Bullet>, asts: &mut Vec<Asteroid>) {
    if p.a {
        ship.rotate(-1);
    } else if p.d {
        ship.rotate(1);
    }
    if p.w {
        ship.accelerate(0.005);
    }
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

use super::asteroid::*;
use super::col::*;
use super::consts::*;
use super::ship::*;
use super::util::*;
use super::explosion::*;

pub fn update(
    ship: &mut Ship,
    p: &Pressed,
    buls: &mut Vec<Bullet>,
    asts: &mut Vec<Asteroid>,
    parts: &mut Particles,
) -> States {
    let sp = &ship.get_points();
    if p.a {
        ship.rotate(-ROTSPEED * 0.8);
    } else if p.d {
        ship.rotate(ROTSPEED * 0.8);
    }
    if p.w {
        ship.accelerate(ACCSPEED / 0.8);
    }
    for ast in asts.iter_mut() {
        if !ast.tick() {
            *ast = Asteroid::new();
        };
        if ship_in_asteroid_circle(sp, ast) {
            if ship_in_asteroid(sp, ast) {
                return States::GameOver;
            }
        }
        for bul in buls.iter() {
            if inside_circle(bul.get_x(), bul.get_y(), ast) {
                //destroy.push(Explosion::new(ast.get_x(), ast.get_y()));
                *ast = Asteroid::new();
                return States::Score;
            }
        }
    }
    for p in parts.iter_mut() {
        p.tick();
    }
    ship.tick();
    *buls = buls
        .iter_mut()
        .filter_map(|bul| return if !bul.tick() { None } else { Some(*bul) })
        .collect();
    States::Nothing
}

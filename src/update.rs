use super::asteroid::*;
use super::col::*;
use super::conf::*;
use super::explosion::*;
use super::randstuff::*;
use super::ship::*;
use super::util::*;

pub fn update(
    ship: &mut Ship,
    p: &Pressed,
    buls: &mut Vec<Bullet>,
    asts: &mut Vec<Asteroid>,
    parts: &mut Particles,
    rr: &mut Ranges,
) -> States {
    let mut tr = States::Nothing;
    let sp = &ship.get_points();
    if p.a {
        ship.rotate(-ROTSPEED);
    } else if p.d {
        ship.rotate(ROTSPEED);
    }
    if p.w {
        ship.accelerate(ACCSPEED / 0.8);
    }
    let mut new: Vec<Asteroid> = Vec::new();
    for ast in asts.iter_mut() {
        if !ast.tick() {
            *ast = Asteroid::new(rr);
        };
        if ship_in_asteroid_circle(sp, ast) {
            if ship_in_asteroid(sp, ast) {
                return States::GameOver;
            }
        }
        for bul in buls.iter() {
            if inside_circle(bul.get_x(), bul.get_y(), ast) {
                //if ast.get_r() <= AST_RAD * (AST_SIZE_VAR * 1.5) {
                //// smol boi
                //} else {
                //// big boi
                //for _ in 0..2 {
                //new.push(Asteroid::new_small(
                //rr,
                //ast.get_x(),
                //ast.get_y(),
                //ast.get_r(),
                //));
                //}
                //}
                for _ in 0..PARTICLES {
                    parts.push(Particle::new(ast.get_x(), ast.get_y(), rr));
                }
                *ast = Asteroid::new(rr);
                tr = States::Score;
            }
        }
    }
    for n in new.iter() {
        asts.push(n.clone());
    }
    *parts = parts
        .iter_mut()
        .filter_map(|part| return if !part.tick() { None } else { Some(*part) })
        .collect();
    ship.tick();
    *buls = buls
        .iter_mut()
        .filter_map(|bul| return if !bul.tick() { None } else { Some(*bul) })
        .collect();
    tr
}

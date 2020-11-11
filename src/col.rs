use super::asteroid::Asteroid;
use super::math::*;
use super::ship::*;
use super::util::*;
// ship + asteroids
pub fn ship_asteroids(ship: &Ship, asts: &[Asteroid; 10]) -> bool {
    for ast in asts.iter() {
        let top: bool = inside_square(
            cos_math(ship.get_size() as f64, 90.0),
            sin_math(ship.get_size() as f64, 90.0),
            ast,
        );
        let left: bool = inside_square(
            cos_math(ship.get_size() as f64, 0.0),
            sin_math(ship.get_size() as f64, 0.0),
            ast,
        );
        let right: bool = inside_square(
            cos_math(ship.get_size() as f64, 180.0),
            sin_math(ship.get_size() as f64, 180.0),
            ast,
        );
        if top && left && right {
            return true;
        }
    }
    false
}

fn inside_square(x: f64, y: f64, ast: &Asteroid) -> bool {
    let sx = ast.get_x();
    let sy = ast.get_y();
    let sw = ast.get_w();
    sx < x && x < sx + sw && sy < y && y < sy + sw
}

pub fn ast_bullet(bullets: &Vec<Bullet>, asts: &[Asteroid; 10]) -> bool {
    for bul in bullets.iter() {
        for ast in asts.iter() {
            // if asteroid inside circle
            for p in &[
                (ast.get_x(), ast.get_y()),
                (ast.get_x() + ast.get_w(), ast.get_y()),
                (ast.get_x(), ast.get_y() + ast.get_w()),
                (ast.get_x() + ast.get_w(), ast.get_y() + ast.get_w()),
            ] {
                if ((bul.get_x() - p.0).powf(2.0) + (bul.get_y() - p.1).powf(2.0)).powf(0.5)
                    < bul.get_rad()
                {
                    return true;
                }
            }
        }
    }
    false
}

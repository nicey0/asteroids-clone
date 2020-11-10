use super::asteroid::Asteroid;
use super::ship::*;
use super::util::Coord;
// ship + asteroids
pub fn el_asts(el: &impl Coord, asts: &[Asteroid; 10]) -> bool {
    for ast in asts.iter() {
        if el_ast(el, ast) {
            return true;
        }
    }
    false
}

pub fn el_ast(el: &impl Coord, ast: &Asteroid) -> bool {
    return if el.get_x() >= ast.get_x()
        && el.get_x() < ast.get_x() + ast.get_w()
        && el.get_y() >= ast.get_y()
        && el.get_y() < ast.get_y() + ast.get_w()
    {
        true
    } else {
        false
    };
}

pub fn ast_bullet(bullets: &Vec<Bullet>, asts: &[Asteroid; 10]) -> bool {
    for bul in bullets.iter() {
        if el_asts(bul, asts) {
            return true;
        }
    }
    false
}

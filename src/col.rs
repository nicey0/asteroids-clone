use super::asteroid::Asteroid;
use super::ship::Ship;
// ship + asteroids
pub fn ship_asteroid(ship: &Ship, asts: &[Asteroid; 10]) -> bool {
    for &ast in asts.iter() {
        if ship.get_x() >= ast.get_x()
            && ship.get_x() < ast.get_x() + ast.get_w()
            && ship.get_y() >= ast.get_y()
            && ship.get_y() < ast.get_y() + ast.get_w()
        {
            return true;
        }
    }
    false
}

use super::asteroid::*;
use super::consts::*;
use super::ship::*;
use super::util::*;
use super::explosion::*;
use piston_window::*;

pub fn render(
    c: &Context,
    g: &mut G2d,
    ship: &Ship,
    buls: &Vec<Bullet>,
    asts: &Vec<Asteroid>,
    parts: &Particles,
) {
    clear([0.0, 0.0, 0.0, 1.0], g);
    draw_polygon(c, g, LINEW, ship.get_points().to_vec());
    for ast in asts.iter() {
        draw_polygon(c, g, LINEW, ast.get_points().to_vec());
        // external collider
        let b = ellipse::Ellipse::new_border([0.0, 1.0, 0.0, 1.0], LINEW);
        b.draw(
            ellipse::circle(ast.get_x(), ast.get_y(), ast.get_r()),
            &DrawState::default(),
            c.transform,
            g,
        );
    }
    let b = ellipse::Ellipse::new_border([1.0; 4], LINEW);
    for bul in buls.iter() {
        b.draw(
            ellipse::circle(bul.get_x(), bul.get_y(), bul.get_r()),
            &DrawState::default(),
            c.transform,
            g,
        );
    }
    for p in parts.iter() {
        b.draw(
            ellipse::circle(p.get_x(), p.get_y(), 0.5),
            &DrawState::default(),
            c.transform,
            g,
        );
    }
}

fn draw_polygon(c: &Context, g: &mut G2d, w: f64, p: Vec<APoint>) {
    for i in 0..p.len() - 1 {
        line_from_to([1.0; 4], w, p[i], p[i + 1], c.transform, g);
    }
    line_from_to([1.0; 4], w, p[0], p[p.len() - 1], c.transform, g);
}

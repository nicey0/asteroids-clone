use super::asteroid::*;
use super::consts::*;
use super::ship::*;
use super::util::*;
use piston_window::*;

pub fn render(
    c: &Context,
    g: &mut G2d,
    ship: &mut Ship,
    buls: &mut Vec<Bullet>,
    asts: &mut Vec<Asteroid>,
) {
    clear([0.0, 0.0, 0.0, 1.0], g);
    draw_polygon(c, g, LINEW, ship.get_points().to_vec());
    for ast in asts.iter_mut() {
        draw_polygon(c, g, LINEW, ast.get_points().to_vec());
    }
    for bul in buls.iter() {
        let b = ellipse::Ellipse::new_border([1.0; 4], LINEW);
        b.draw(
            ellipse::circle(bul.get_x(), bul.get_y(), bul.get_r()),
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

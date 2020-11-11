use super::asteroid::*;
use super::consts::*;
use super::math::*;
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
    draw_triangle(
        c,
        g,
        [0.0, 1.0],
        [DIM as f64 / 2.0, DIM as f64],
        [DIM as f64, 1.0],
    );
}

fn draw_polygon(c: &Context, g: &mut G2d, p: Vec<APoint>) {
    for i in (0..p.len()).step_by(2) {
        line_from_to([1.0; 4], 1.0, p[i], p[i + 1], c.transform, g);
    }
}

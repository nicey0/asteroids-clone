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
        [0.0, 0.0],
        [DIM as f64 / 2.0, DIM as f64],
        [DIM as f64, 0.0],
    );
}

fn draw_triangle(c: &Context, g: &mut G2d, ta: APoint, tb: APoint, tc: APoint) {
    for &(q, w) in &[(ta, tb), (tb, tc), (tc, ta)] {
        line_from_to([1.0, 1.0, 1.0, 1.0], 1.0, q, w, c.transform, g);
    }
}

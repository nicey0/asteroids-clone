use rand::{
    distributions::{Distribution, Uniform},
    thread_rng, Rng,
};

use super::consts::*;

struct Ranges {
    pub zero_one: Uniform<u8>,

    pub dim_half: Uniform<f64>,

    pub ast_edges: Uniform<u8>,
    pub ast_r: Uniform<f64>,
    pub ast_speed: Uniform<f64>,
}

impl Ranges {
    pub fn create() -> Self {
        Self {
            zero_one: Uniform::from(0..=1),
            dim_half: Uniform::from(0.0..=DIM as f64),
            ast_edges: Uniform::from(AST_EDGES / 2..=AST_EDGES),
            ast_r: Uniform::from(AST_RAD * AST_SIZE_VAR..=AST_RAD),
            ast_speed: Uniform::from(ASTSPD.0..=ASTSPD.1),
        }
    }
}

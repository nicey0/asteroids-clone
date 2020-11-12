use std::f64::consts::PI;

// screen
pub const DIM: u32 = 600;
pub const PADDING: u32 = 30;

// rendering
pub const LINEW: f64 = 0.4;

// speed
pub const MAXSPEED: f64 = 1.5;
pub const ASTSPD: (f64, f64) = (0.2, 0.6);
pub const BULSPD: f64 = 3.0;

// size
pub const BULLEN: f64 = 5.0;

// asteroids
pub const AST_EDGES: u8 = 10;
pub const AST_COUNT: u8 = 6;

// math
pub const RADTODEG: f64 = 2.0 * (PI / 360.0);

use std::f64::consts::PI;

// screen
pub const DIM: u32 = 600;
pub const PADDING: u32 = 15;

// rendering
pub const LINEW: f64 = 0.4;

// size
pub const BULRAD: f64 = 0.3;
pub const FSIZE: u32 = 20;

// ship
pub const SHIPSIZE: f64 = 5.0;
pub const MAXSPEED: f64 = 100.0;
pub const ACCSPEED: f64 = 0.02;
pub const ROTSPEED: f64 = 4.0;

// bullets
pub const BULCOOLDOWN: u8 = 15;
pub const BULSPD: f64 = 5.0;

// asteroids
pub const ASTSPD: (f64, f64) = (0.4, 1.0);
pub const AST_RAD: f64 = 30.0;
pub const AST_SIZE_VAR: f64 = 0.4;
pub const AST_EDGES: u8 = 25;
pub const AST_COUNT: usize = 5;
pub const AST_ROUND: f64 = 0.7;

// particles
pub const PARTICLES: u8 = 10;

// math
pub const RADTODEG: f64 = 2.0 * (PI / 360.0);

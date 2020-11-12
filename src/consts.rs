use std::f64::consts::PI;

// screen
pub const DIM: u32 = 600;
pub const PADDING: u32 = 30;

// rendering
pub const LINEW: f64 = 0.4;

// size
pub const BULRAD: f64 = 2.0;
pub const FSIZE: u32 = 22;

// ship
pub const MAXSPEED: f64 = 100.0;
pub const ACCSPEED: f64 = 0.08;
pub const ROTSPEED: f64 = 4.0;

// bullets
pub const BULCOOLDOWN: u8 = 15;
pub const BULSPD: f64 = 10.0;

// asteroids
pub const ASTSPD: (f64, f64) = (1.0, 2.0);
pub const AST_RAD: f64 = PADDING as f64 * 2.0;
pub const AST_SIZE_VAR: f64 = 0.5;
pub const AST_EDGES: u8 = 15;
pub const AST_COUNT: u8 = 5;
pub const AST_ROUND: f64 = 0.4;

// math
pub const RADTODEG: f64 = 2.0 * (PI / 360.0);

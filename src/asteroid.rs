pub struct Asteroid {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    xspd: f64,
    yspd: f64,
}

impl Asteroid {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            xspd: 0.0,
            yspd: 0.0,
        }
    }

    fn get_random_x() -> (f64, f64) {
        0.0, 0.0;
    }

    pub fn tick(&mut self) -> bool {
        self.x += self.xspd;
        self.y += self.yspd;
        // check out of bounds
        !(x < 0.0 || x > WIDTH + PADDING || y < 0.0 || y > HEIGHT + PADDING)
    }
}

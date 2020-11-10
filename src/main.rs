use std::f64::consts::PI;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;
const RADTODEG: f64 = 2.0 * (PI / 360.0);

#[derive(Debug)]
struct Ship {
    x: f64,
    y: f64,
    spd: f64,
    rot: f64,
}

impl Ship {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            spd: 0.0,
            rot: 0.0,
        }
    }

    fn rotate(&mut self, deg: f64) {
        self.rot += deg;
    }

    fn set_speed(&mut self, spd: f64) {
        self.spd = spd;
    }

    fn tick(&mut self) {
        self.x += self.spd * (self.rot * RADTODEG).cos();
        self.y += self.spd * (self.rot * RADTODEG).sin();
    }
}

fn main() {
    let mut ship = Ship::new();
    ship.rotate(90.0);
    ship.set_speed(5.0);
    ship.tick();
    println!("{:#?}", ship);
    ship.tick();
    println!("{:#?}", ship);
    ship.tick();
    println!("{:#?}", ship);
    ship.rotate(-90.0);
    ship.tick();
    println!("{:#?}", ship);
}

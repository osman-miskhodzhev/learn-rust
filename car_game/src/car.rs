use std::f64::consts::PI;

struct Car {
    pos: Vec2,
    angle: f32,
    vel: f32,
    steer: f32,

    l: f32,
    max_steer: f32,
    max_speed: f32,
}

impl Car {
    pub fn new(x: f32, y:f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            angle: 0.0,
            vel: 0.0,
            steer: 0.0,

            l: 2.5,
            max_steer: 30.0_f32.to_radians(),
            max_speed: 10.0,
        }
    }
}
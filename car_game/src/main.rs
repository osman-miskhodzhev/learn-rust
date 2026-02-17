use macroquad::prelude::*;

struct Car {
    pos: Vec2,
    radius: f32,
    speed: f32,
    speed_boost: f32,
}

impl Car {
    fn new(x: f32, y: f32) -> Self {
        Self { pos: Vec2::new(x, y), radius: 2.0, speed: 2.0, speed_boost: 0.01}
    }

    fn get_speed(&mut self) -> f32 {
        self.speed += self.speed_boost;
        self.speed
    }

    fn update(&mut self) {
        if is_key_down(KeyCode::Right) { self.pos.x += self.get_speed(); }
        if is_key_down(KeyCode::Left) { self.pos.x -= self.get_speed(); }
        if is_key_down(KeyCode::Up) { self.pos.y -= self.get_speed(); }
        if is_key_down(KeyCode::Down) { self.pos.y += self.get_speed(); }

        self.pos.x = self.pos.x.clamp(self.radius, screen_width() - self.radius);
        self.pos.y = self.pos.y.clamp(self.radius, screen_height() - self.radius);
    }

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, RED);
    }
}

#[macroquad::main("Simple")]
async fn main() {
    let mut car = Car::new(100.0, 350.0);
    loop {
        clear_background(BLACK);
        car.update();
        car.draw();
        next_frame().await;
    }
}

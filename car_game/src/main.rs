use macroquad::prelude::*;

struct Car {
    pos: Vec2,
    radius: f32,
    base_speed: f32, // Обычная скорость
    boost: f32,      // Добавка к скорости
}

impl Car {
    fn new(x: f32, y: f32) -> Self {
        Self { pos: Vec2::new(x, y), radius: 2.0, base_speed: 2.0, boost: 5.0}
    }

    fn update(&mut self) {
        // Вычисляем скорость только для этого кадра
        let current_speed = if is_key_down(KeyCode::LeftShift) {
            self.base_speed + self.boost
        } else {
            self.base_speed
        };

        if is_key_down(KeyCode::Right) { self.pos.x += current_speed; }
        if is_key_down(KeyCode::Left) { self.pos.x -= current_speed; }
        if is_key_down(KeyCode::Up) { self.pos.y -= current_speed; }
        if is_key_down(KeyCode::Down) { self.pos.y += current_speed; }

        // Ограничение экраном
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

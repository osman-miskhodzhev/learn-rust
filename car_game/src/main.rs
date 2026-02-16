
use macroquad::prelude::*;

struct Car {
    pos: Vec2,
    radius: f32,
    speed: f32,
    boost: f32,
}

impl Car {
    fn new(x: f32, y: f32) -> Self {
        Self { pos: Vec2::new(x, y), radius: 20.0, speed: 2.0, boost: 0.1 }
    }

    fn update(&mut self, obstacles: &[Rect]) {
        // Ускорение
        if is_key_down(KeyCode::LeftShift) {
            self.speed += self.boost;
        }

        let mut next_pos = self.pos;
        if is_key_down(KeyCode::Right) { next_pos.x += self.speed; }
        if is_key_down(KeyCode::Left) { next_pos.x -= self.speed; }
        if is_key_down(KeyCode::Up) { next_pos.y -= self.speed; }
        if is_key_down(KeyCode::Down) { next_pos.y += self.speed; }

        // Проверка столкновений
        let mut collide = false;
        for rect in obstacles {
            let closest = Vec2::new(
                next_pos.x.clamp(rect.x, rect.x + rect.w),
                next_pos.y.clamp(rect.y, rect.y + rect.h),
            );
            if next_pos.distance_squared(closest) < self.radius * self.radius {
                collide = true;
                break;
            }
        }

        if !collide { self.pos = next_pos; }
    }

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, GREEN);
    }
}

#[macroquad::main("Collision")]
async fn main() {
    let mut car = Car::new(100.0, 350.0);
    let obstacles = [
        Rect::new(300.0, 100.0, 100.0, 200.0),
        Rect::new(500.0, 400.0, 200.0, 50.0),
    ];

    loop {
        clear_background(BLACK);
        for rect in &obstacles {
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, RED);
        }
        
        car.update(&obstacles);
        car.draw();
        
        next_frame().await;
    }
}
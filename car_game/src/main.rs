use macroquad::prelude::*;

const ROAD_WIDTH: f32 = 300.0;
const LANE_COUNT: usize = 3;
const LANE_HEIGHT: f32 = 80.0;
const LANE_GAP: f32 = 20.0;

struct Car {
    speed: f32,
    y: f32,
}

impl Car {
    fn new() -> Self {
        Self { speed: 0.0, y: 0.0 }
    }

    fn update(&mut self) {
        if is_key_down(KeyCode::W) {
            self.speed += 0.2;
        }
        if is_key_down(KeyCode::S) {
            self.speed -= 0.2;
        }
        self.speed *= 0.95; // лёгкое трение
        self.speed = self.speed.clamp(-5.0, 15.0);
        self.y += self.speed;
    }

    fn draw(&self) {
        // Машина всегда в центре экрана
        let screen_x = screen_width() / 2.0 - 20.0;
        let screen_y = screen_height() / 2.0 - 35.0;
        draw_rectangle(screen_x, screen_y, 40.0, 70.0, RED);
    }
}

#[macroquad::main("Car Game")]
async fn main() {
    let mut car = Car::new();
    let mut road_offset: f32 = 0.0;

    loop {
        clear_background(DARKGRAY);

        car.update();

        // Обновляем смещение полос с учётом скорости
        road_offset = (road_offset + car.speed) % (LANE_HEIGHT + LANE_GAP);

        // Рисуем дорогу по центру экрана
        let road_left = screen_width() / 2.0 - ROAD_WIDTH / 2.0;
        draw_rectangle(road_left, 0.0, ROAD_WIDTH, screen_height(), GRAY);

        // Рисуем разметку (движущиеся полосы)
        let mut y = -road_offset;
        while y < screen_height() {
            draw_rectangle(
                screen_width() / 2.0 - 5.0,
                y,
                10.0,
                LANE_HEIGHT,
                WHITE,
            );
            y += LANE_HEIGHT + LANE_GAP;
        }

        car.draw();

        next_frame().await;
    }
}
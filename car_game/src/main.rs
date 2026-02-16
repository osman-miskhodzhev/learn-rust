
use macroquad::prelude::*;

// Проверка столкновения круга и прямоугольника
fn check_collision(pos: Vec2, radius: f32, rect: Rect) -> bool {
    let closest = Vec2::new(
        pos.x.clamp(rect.x, rect.x + rect.w),
        pos.y.clamp(rect.y, rect.y + rect.h),
    );
    pos.distance_squared(closest) < radius * radius
}

#[macroquad::main("Collision")]
async fn main() {
    let mut pos = Vec2::new(100.0, 350.0);
    let radius = 20.0; // Шарик меньше
    let speed = 2.0;

    // Препятствия (x, y, ширина, высота)
    let obstacles = [
        Rect::new(300.0, 100.0, 100.0, 200.0),
        Rect::new(500.0, 400.0, 200.0, 50.0),
    ];

    loop {
        let mut next_pos = pos;

        if is_key_down(KeyCode::Right) { next_pos.x += speed; }
        if is_key_down(KeyCode::Left) { next_pos.x -= speed; }
        if is_key_down(KeyCode::Up) { next_pos.y -= speed; }
        if is_key_down(KeyCode::Down) { next_pos.y += speed; }

        // Проверка столкновений
        let mut collide = false;
        for rect in &obstacles {
            if check_collision(next_pos, radius, *rect) {
                collide = true;
                break;
            }
        }

        if !collide { pos = next_pos; }

        clear_background(BLACK);
        
        // Рисуем препятствия
        for rect in &obstacles {
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, RED);
        }
        
        // Рисуем игрока
        draw_circle(pos.x, pos.y, radius, GREEN);
        
        next_frame().await;
    }
}

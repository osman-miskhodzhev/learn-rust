use macroquad::prelude::*;
mod car;
use car::Car;

#[macroquad::main("Аркадные гонки")]
async fn main() {
    let mut car = Car::new(400.0, 300.0).await;
    
    loop {
        clear_background(DARKGRAY);
        
        let dt = get_frame_time();
        car.update(dt);
        car.draw();
        
        // Рисуем простую трассу
        draw_rectangle(0.0, 0.0, screen_width(), 5.0, WHITE);
        draw_rectangle(0.0, screen_height()-5.0, screen_width(), 5.0, WHITE);
        draw_rectangle(0.0, 0.0, 5.0, screen_height(), WHITE);
        draw_rectangle(screen_width()-5.0, 0.0, 5.0, screen_height(), WHITE);
        
        // Альтернативный способ отображения текста
        let fps_text = format!("FPS: {}", get_fps());
        draw_text(&fps_text, 10.0, 70.0, 20.0, GREEN);
        
        next_frame().await;
    }
}
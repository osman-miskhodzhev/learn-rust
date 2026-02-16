use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    let mut pos = Vec2::new(400.0, 300.0);
    loop {
        clear_background(BLACK);
        
        if is_key_down(KeyCode::Right) { pos.x += 2.0; }
        if is_key_down(KeyCode::Left) { pos.x -= 2.0; }
        if is_key_down(KeyCode::Up) { pos.y -= 2.0; }
        if is_key_down(KeyCode::Down) { pos.y += 2.0; }

        draw_circle(pos.x, pos.y, 50.0, GREEN);
        next_frame().await;
    }
}
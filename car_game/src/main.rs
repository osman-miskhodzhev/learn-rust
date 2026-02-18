use macroquad::prelude::*;

struct Car {
    pos: Vec2,
    radius: f32,
    base_speed: f32,
    boost: f32,
    texture: Texture2D,  // Добавляем поле для текстуры
}

impl Car {
    async fn new(x: f32, y: f32) -> Self {  // Теперь функция асинхронная
        // Загружаем текстуру (путь к файлу изображения)
        let texture = load_texture("car.png").await.unwrap();
        
        Self { 
            pos: Vec2::new(x, y), 
            radius: 2.0, 
            base_speed: 2.0, 
            boost: 1.0,
            texture,  // Сохраняем текстуру
        }
    }

    fn update(&mut self) {
        let current_speed = if is_key_down(KeyCode::LeftShift) {
            self.base_speed + self.boost
        } else {
            self.base_speed
        };

        if is_key_down(KeyCode::Right) { self.pos.x += current_speed; }
        if is_key_down(KeyCode::Left) { self.pos.x -= current_speed; }
        if is_key_down(KeyCode::Up) { self.pos.y -= current_speed; }
        if is_key_down(KeyCode::Down) { self.pos.y += current_speed; }

        self.pos.x = self.pos.x.clamp(self.radius, screen_width() - self.radius);
        self.pos.y = self.pos.y.clamp(self.radius, screen_height() - self.radius);
    }

    fn draw(&self) {
       let scale_x = 0.1;  // Масштаб по X
    let scale_y = 0.1;  // Масштаб по Y
    
    draw_texture_ex(
        &self.texture,
        self.pos.x - (self.texture.width() * scale_x) / 2.0,
        self.pos.y - (self.texture.height() * scale_y) / 2.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(
                self.texture.width() * scale_x,
                self.texture.height() * scale_y
            )),
            ..Default::default()
        }
    );
    }
}

#[macroquad::main("Simple")]
async fn main() {
    let mut car = Car::new(100.0, 350.0).await;  // Ждем загрузки текстуры
    
    loop {
        clear_background(BLACK);
        car.update();
        car.draw();
        next_frame().await;
    }
}
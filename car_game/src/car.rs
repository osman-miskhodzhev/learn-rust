use macroquad::prelude::*;

pub struct Car {
    pos: Vec2,
    radius: f32,
    base_speed: f32,
    boost: f32,
    angle: f32,
    rotation_speed: f32,
    texture: Texture2D,
}

impl Car {
    pub async fn new(x: f32, y: f32) -> Self {
        let texture = load_texture("car.png").await.unwrap();
        
        Self { 
            pos: Vec2::new(x, y), 
            radius: 2.0, 
            base_speed: 2.0, 
            boost: 1.0,
            angle: 0.0,
            rotation_speed: 0.5,
            texture,
        }
    }

    pub fn update(&mut self) {
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

    pub fn draw(&self) {
        let scale_x = 0.1;
        let scale_y = 0.1;
        
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
use macroquad::prelude::*;

pub struct Car {
    pos: Vec2,
    speed: f32,           // текущая скорость (пикс/сек)
    base_speed: f32,      // базовая скорость (пикс/сек)
    boost_multiplier: f32, // множитель ускорения
    angle: f32,           // угол поворота машины (радианы)
    rotation_speed: f32,  // скорость поворота (рад/сек)
    len: f32,
    texture: Texture2D,
}

impl Car {
    pub async fn new(x: f32, y: f32) -> Self {
        let texture = load_texture("car.png").await.unwrap();
        let height = texture.height() * 0.1;
        
        Self { 
            pos: Vec2::new(x, y),
            speed: 0.0,                    // начинаем с 0 скорости
            base_speed: 500.0,              // 300 пикселей в секунду
            boost_multiplier: 2.0,           // x2 скорость с шифтом
            angle: 0.0,
            rotation_speed: 2.5,             // 2.5 радиан в секунду (~143°/сек)
            texture,
            len: height,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // ===== УПРАВЛЕНИЕ ПОВОРОТОМ =====
        if is_key_down(KeyCode::Right) {
            self.angle += self.rotation_speed * dt;  // поворот направо
        }
        if is_key_down(KeyCode::Left) {
            self.angle -= self.rotation_speed * dt;  // поворот налево
        }
        
        // Нормализуем угол (опционально, чтобы не рос бесконечно)
        // self.angle = self.angle % (2.0 * std::f32::consts::PI);
        
        // ===== УПРАВЛЕНИЕ СКОРОСТЬЮ =====
        // Определяем максимальную скорость с учетом буста
        let max_speed = if is_key_down(KeyCode::LeftShift) {
            self.base_speed * self.boost_multiplier
        } else {
            self.base_speed
        };
        
        // Газ/тормоз
        if is_key_down(KeyCode::Up) {
            // Разгон до max_speed
            self.speed += 800.0 * dt;  // ускорение 800 пикс/сек²
            if self.speed > max_speed {
                self.speed = max_speed;
            }
        } else if is_key_down(KeyCode::Down) {
            // Торможение или задний ход
            self.speed -= 1000.0 * dt;  // торможение 1000 пикс/сек²
            if self.speed < -max_speed * 0.5 {  // задний ход в 2 раза медленнее
                self.speed = -max_speed * 0.5;
            }
        } else {
            // Естественное замедление (трение)
            if self.speed > 0.0 {
                self.speed -= 400.0 * dt;
                if self.speed < 0.0 {
                    self.speed = 0.0;
                }
            } else if self.speed < 0.0 {
                self.speed += 400.0 * dt;
                if self.speed > 0.0 {
                    self.speed = 0.0;
                }
            }
        }
        
        // ===== ДВИЖЕНИЕ =====
        if self.speed != 0.0 {
            // Движемся в направлении угла машины
            self.pos.x += self.speed * self.angle.sin() * dt;
            self.pos.y -= self.speed * self.angle.cos() * dt;
        }
        
        // ===== ОГРАНИЧЕНИЯ (чтобы не улететь за экран) =====
        // Получаем размеры экрана
        let screen_width = screen_width();
        let screen_height = screen_height();
        
        // Границы с учетом размера машины
        let car_width = self.texture.width() * 0.1;
        let car_height = self.texture.height() * 0.1;
        
        self.pos.x = self.pos.x.clamp(car_width/2.0, screen_width - car_width/2.0);
        self.pos.y = self.pos.y.clamp(car_height/2.0, screen_height - car_height/2.0);
    }

    pub fn draw(&self) {
        let scale_x = 0.1;
        let scale_y = 0.1;
        
        // Рисуем машину с поворотом
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
                rotation: self.angle,
                ..Default::default()
            }
        );
        
        // Отрисовка отладочной информации
        draw_text(
            &format!("Скорость: {:.0} пикс/сек", self.speed),
            10.0, 30.0, 20.0, YELLOW
        );
        
        draw_text(
            &format!("Угол: {:.0}°", self.angle.to_degrees()),
            10.0, 50.0, 20.0, YELLOW
        );
        
        // Вектор направления (для наглядности)
        let line_end = Vec2::new(
            self.pos.x + 30.0 * self.angle.cos(),
            self.pos.y + 30.0 * self.angle.sin()
        );
        draw_line(self.pos.x, self.pos.y, line_end.x, line_end.y, 2.0, RED);
    }
    
    // Геттеры для позиции (если нужны в main)
    pub fn position(&self) -> Vec2 {
        self.pos
    }
    
    pub fn speed(&self) -> f32 {
        self.speed
    }
}
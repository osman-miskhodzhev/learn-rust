pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    pub fn info(&self) {
        println!("Прямоугольник {}x{}", self.width, self.height);
        println!("Его площадь равна {}", self.area());
        println!("===================================");
    }
}

pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn area(&self) -> f32 {
        2*std::f32::consts:PI * self.radius * self.radius
    }
}


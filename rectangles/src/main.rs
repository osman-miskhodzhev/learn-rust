struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    fn info(&self) {
        println!(
            "Прямоугольник {}x{}",
            self.width, self.height
        );
        
        println!(
            "Его площадь равна {}",
            self.area()
        );
        println!("===================================");
    }
}


fn main() {
    
    println!("===================================");
    println!("          Начало программы");
    println!("===================================");
    let rect = Rectangle {
        width: 30.0,
        height: 30.0,
    };
    rect.info();
}   


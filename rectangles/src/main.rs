struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
    
}

fn main() {
    let rect = Rectangle {
        width: 30.0,
        height: 30.0,
    };
    println!(
        "Результат работы программы"
    );
    println!(
        "S = {} * {} = {}",
        rect.width,
        rect.height,
        rect.area()
    );
}


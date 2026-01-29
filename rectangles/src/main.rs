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

struct Box {
    width: u32,
    height: u32,
    depth: u32,
}

impl Box {
    fn volume(&self) -> u32 {
        self.width * self.height * self.depth
    }
    
    fn area(&self) -> u32 {
        2 * (
            self.width * self.height +
            self.width * self.depth +
            self.height * self.depth
        )
    }
    fn info(&self) {
        println!("Параллепипед {}x{}x{}", self.width, self.height, self.depth);
        println!("Его объем равен {}", self.volume());
        println!("Его площадь равна {}", self.area());
        println!("===================================");
    }
}

struct Circle {
    radius: f64
}
impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn info(&self) {
        println!("Это окружность с радиусом {} и площадью {}", self.radius, self.area())
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
    

    
    let cube = Box {
        width: 300,
        height: 450,
        depth: 100
    };

    cube.info();
    
    let circle = Circle {radius: 30.0};
    circle.info();
}   


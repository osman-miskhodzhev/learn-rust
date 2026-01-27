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
}

fn main() {
    let rect = Rectangle {
        width: 30.0,
        height: 30.0,
    };
    
    let cube = Box {
        width: 300,
        height: 450,
        depth: 100
    };
    
    println!(
        "Расчет объема. V = {} * {} * {} = {}",
        cube.width,
        cube.height,
        cube.depth,
        cube.volume()
    );

    println!(
        "Результат работы программы"
    );
    println!(
        "S = {} * {} = {}",
        rect.width,
        rect.height,
        rect.area()
    );
    
    let rect1 = Rectangle {
        width: 150.0,
        height: 150.0
    };
    let rect2 = Rectangle {
        width: 100.0,
        height: 3.0
    };
    
    if rect2.can_hold(&rect1) {
        println!(
            "Прямоугольник с площадью {}, может поместитья в прямоугольник с площадью {}",
            rect1.area(),
            rect2.area()
        );
    } else {
        println!(
            "Прямоугольник с площадью {}, не может поместитья в прямоугольник с площадью {}",
            rect1.area(),
            rect2.area()
        );
    };
}   


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
}

fn main() {
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

    println!(
        "Площадь поверхности параллепипеда {} x {} x {} равна {}",
        cube.width,
        cube.height,
        cube.depth,
        cube.area()
    );
    
    println!(
        "Расчет объема. V = {} * {} * {} = {}",
        cube.width,
        cube.height,
        cube.depth,
        cube.volume()
    );
}   


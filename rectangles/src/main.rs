mod shape;

use shape::Rectangle;
use shape::Circle;

fn main() {
    
    println!("===================================");
    println!("          Начало программы");
    println!("===================================");
    let rect = Rectangle {
        width: 30.0,
        height: 30.0,
    };
    rect.info();


    let circle1 = Circle {
        radius: 40.0,
    };
    
    println!("Площадь круга радиусом в {} см. равна {}", circle1.radius, circle1.area());
}   


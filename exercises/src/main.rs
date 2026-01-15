fn greet(name: &str) -> String{
    format!("Hello, {}!", name)
}

fn rectangle_area(width: f64, heigth: f64) -> f64 {
    width * heigth
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    println!("ex 1: ");
    println!("{}", greet("Osman"));
    println!("ex 2: ");
    println!("{}", rectangle_area(5.0, 3.0)); 
    println!("ex 3: ");
    println!("{}", is_even(4));
}

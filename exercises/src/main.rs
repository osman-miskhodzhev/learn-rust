fn greet(name: &str) -> String{
    format!("Hello, {}!", name)
}

fn rectangle_area(width: f64, heigth: f64) -> f64 {
    width * heigth
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn min_max(values: &[i32]) -> (i32, i32) {
    let mut min = values[0];
    let mut max = values[0];
    
    for &val in values {
        if val < min {
	    min=val;
	}
	if val > max {
	    max=val;
	}
    }
    (min, max)
}

fn main() {
    println!("ex 1: ");
    println!("{}", greet("Osman"));
    println!("ex 2: ");
    println!("{}", rectangle_area(5.0, 3.0)); 
    println!("ex 3: ");
    println!("{}", is_even(4));
    
    println!("ex 4: ");
    let nums = [3, 1, 4, 1, 5, 9];
    let (min, max) = min_max(&nums);
    println!("Min: {}, Max: {}", min, max);
}

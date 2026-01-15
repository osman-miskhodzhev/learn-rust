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

fn factorial(n: u32) -> u64{
    if n == 0 {
	1
    } else {
	n as u64 * factorial(n-1)
    }
}

fn binary_search(elems: &[i32], n:i32) -> Option<usize> {
    let mut left = 0;
    let mut right = elems.len();

    while left < right {
	let mid = left + (right - left)/2;
	if elems[mid] == n {
	    return Some(mid);
	} else if elems[mid] < n{
	    left = mid + 1;
	} else {
	    right = mid;
	}
    }
    None
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

    println!("ex 5: ");
    println!("{}", factorial(5));

    let arr = [1, 3, 5, 7, 9];
    println!("{:?}", binary_search(&arr, 5)); // Some(2)
    println!("{:?}", binary_search(&arr, 4)); // None
    println!("{:?}", binary_search(&[], 1));  // None
}

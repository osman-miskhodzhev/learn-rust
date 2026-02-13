mod utils;

use utils::Rectangle;

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


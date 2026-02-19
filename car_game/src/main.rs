use macroquad::prelude::*;
mod car;
use car::Car;

#[macroquad::main("Simple")]
async fn main() {
    let mut car = Car::new(100.0, 350.0).await;
    
    loop {
        clear_background(GRAY);
        car.update();
        car.draw();
        next_frame().await;
    }
}
use std::io;

fn main() {
    println!("Введите N:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");
    
    let n: u32 = input.trim().parse().expect("Введите корректное число");
    
    let mut sum = 0;
    for i in 1..=n {
        if i % 2 == 0 {
            sum += i;
        }
    }
    
    println!("Сумма чётных чисел от 1 до {} = {}", n, sum);
}

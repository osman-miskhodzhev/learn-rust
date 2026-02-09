enum Message {
    Text(String),
    Number(i32),
}
fn process_msg(message: Message) {
    match message {
        Message::Text(s) => {println!("Текстовое сообщение. Содержание: {}", s);},
        Message::Number(n) => {println!("Номер. Содержание: {}", n);},
    }
}

fn main() {
    let msg1 = Message::Text(String::from("Hello!"));
    let msg2 = Message::Number(50);
    
    process_msg(msg1);
    process_msg(msg2);
}

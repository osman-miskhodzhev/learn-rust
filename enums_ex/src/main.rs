enum Message {
    Text(String),
    Number(i32),
}
fn main() {
    let msg1 = Message::Text(String::from("Hello!"));
    let msg2 = Message::Number(50);
}

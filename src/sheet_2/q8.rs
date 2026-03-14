enum Message {
    Move { x: i32, y: i32 },
    Email(String),
}

fn sum_of_message(msg: &Message) -> i32 {
    match msg {
        Message::Move { x, y } => x + y,
        _ => 0,
    }
}

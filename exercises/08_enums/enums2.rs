#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    // TODO: 아래에서 사용하는 다양한 변형(variant)들을 정의해봐!
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}

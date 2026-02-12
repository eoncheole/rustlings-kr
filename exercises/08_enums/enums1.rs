#[derive(Debug)]
enum Message {
    // TODO: 아래에서 사용하는 것처럼 몇 가지 메시지 타입을 정의해봐!
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}

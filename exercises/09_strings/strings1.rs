// TODO: 함수 시그니처(function signature)를 바꾸지 않고 컴파일러 에러를 고쳐봐!
fn current_favorite_color() -> String {
    "blue"
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}

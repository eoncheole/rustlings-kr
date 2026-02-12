// TODO: 시그니처는 바꾸지 말고 함수 본문을 고쳐봐!
fn square(num: i32) -> i32 {
    num * num;
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}

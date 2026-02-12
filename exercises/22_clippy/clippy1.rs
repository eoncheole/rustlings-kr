// Clippy 도구는 코드를 분석해서 흔한 실수를 잡아주고 Rust 코드를
// 개선할 수 있게 도와주는 린트(Lint) 모음이야.
//
// 이 연습문제들에서는 Clippy 경고가 있으면 컴파일이 실패해.
// 출력에서 Clippy의 제안을 확인해서 문제를 풀어봐!

fn main() {
    // TODO: 이 줄의 Clippy 린트를 고쳐봐!
    let pi = 3.14;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("The area of a circle with radius {radius:.2} is {area:.5}");
}

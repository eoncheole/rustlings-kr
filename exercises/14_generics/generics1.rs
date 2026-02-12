// `Vec<T>`는 타입 `T`에 대해 제네릭(Generic)해. 대부분의 경우 컴파일러가
// `T`를 추론할 수 있어, 예를 들어 구체적인 타입의 값을 벡터에 push한 후에 말이야.
// 하지만 이 연습문제에서는 타입 어노테이션(type annotation)으로 컴파일러를 도와줘야 해.

fn main() {
    // TODO: 벡터 `Vec<T>`의 타입을 어노테이션해서 컴파일러 에러를 고쳐봐!
    // `u8`과 `i8`로부터 만들 수 있는 정수 타입을 `T`로 선택해봐.
    let mut numbers = Vec::new();

    // 아래 줄은 변경하지 마.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}

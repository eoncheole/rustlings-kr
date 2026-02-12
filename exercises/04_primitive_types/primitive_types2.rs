// 문자 (`char`)

fn main() {
    // _작은따옴표_를 사용하고 있다는 점에 주목해. 지금까지 봐왔던 큰따옴표와는 달라.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // TODO: 위의 예시처럼, 아래에 좋아하는 문자로 `your_character`라는 변수를 선언해봐.
    // 글자, 숫자(작은따옴표 안에), 특수문자, 다른 언어의 문자, 이모지 😉 등을 시도해봐!
    // let your_character = '';

    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}

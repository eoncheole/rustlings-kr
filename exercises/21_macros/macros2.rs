fn main() {
    my_macro!();
}

// TODO: 이 매크로(Macro)의 전체 정의를 옮겨서 컴파일러 에러를 고쳐봐!
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

// TODO: 매크로(Macro) 정의를 이 모듈 밖으로 꺼내지 않고 컴파일러 에러를 고쳐봐!
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}

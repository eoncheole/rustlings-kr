// Rust 컴파일러는 제공된 참조(reference)가 유효한지 검사하는 방법을 알아야 해.
// 그래야 참조가 사용되기 전에 스코프를 벗어날 위험이 있는지 프로그래머에게
// 알려줄 수 있거든. 참조는 빌림(borrow)이고 자기 데이터를 소유하지 않는다는 걸
// 기억해! 만약 소유자가 스코프를 벗어나면 어떻게 될까?

// TODO: 함수 시그니처를 수정해서 컴파일러 에러를 고쳐봐.
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}

fn factorial(num: u64) -> u64 {
    // TODO: `num`의 팩토리얼(factorial)을 반환하는 함수를 완성해봐.
    // 팩토리얼은 `1 * 2 * 3 * … * num`으로 정의돼.
    // https://en.wikipedia.org/wiki/Factorial
    //
    // 사용하지 말아야 할 것:
    // - 조기 반환 (`return` 키워드를 명시적으로 사용하는 것)
    // 가능하면 사용하지 않아볼 것:
    // - 명령형 스타일 루프 (for/while)
    // - 추가 변수
    // 추가 도전으로, 이것도 사용하지 않아봐:
    // - 재귀(recursion)
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}

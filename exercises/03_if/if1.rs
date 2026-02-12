fn bigger(a: i32, b: i32) -> i32 {
    // TODO: 이 함수를 완성해서 더 큰 숫자를 반환해봐!
    // 두 숫자가 같으면 아무거나 반환하면 돼.
    // 다음은 사용하지 마:
    // - 다른 함수 호출
    // - 추가 변수
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

// 이건 지금은 신경 쓰지 마 :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}

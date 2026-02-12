// 테스트(tests)는 네 코드가 네가 생각한 대로 동작하는지 확인하는 데 정말 중요해.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    // TODO: `is_even`을 가져와(import). 와일드카드(wildcard)를 사용해서 바깥 모듈의
    // 모든 것을 가져올 수 있어.

    #[test]
    fn you_can_assert() {
        // TODO: `is_even` 함수를 몇 가지 값으로 테스트해봐.
        assert!();
        assert!();
    }
}

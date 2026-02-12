// 비트 시프트(bit shift)를 사용해서 2의 거듭제곱을 계산해.
// `1 << n`은 "2의 n제곱"과 같아.
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO: `power_of_2` 함수를 몇 가지 값으로 테스트해봐.
        assert_eq!();
        assert_eq!();
        assert_eq!();
        assert_eq!();
    }
}

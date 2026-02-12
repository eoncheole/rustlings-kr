// 이 함수는 냉장고에 아이스크림이 얼마나 남았는지 반환해.
// 22:00(24시간제) 이전이면 5스쿱이 남아있어. 22:00에 누군가 다 먹어버려서
// 아이스크림이 하나도 안 남아 (값 0). `hour_of_day`가 23보다 크면 `None`을
// 반환해야 해.
fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    // TODO: 함수 본문을 완성해봐.
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO: 이 테스트를 고쳐봐. Option 안에 들어있는 값을 어떻게 꺼낼 수
        // 있을까?
        let ice_creams = maybe_ice_cream(12);

        assert_eq!(ice_creams, 5); // 이 줄은 바꾸지 마.
    }

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(0), Some(5));
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(18), Some(5));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(24), None);
        assert_eq!(maybe_ice_cream(25), None);
    }
}

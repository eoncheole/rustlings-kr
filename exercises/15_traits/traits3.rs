trait Licensed {
    // TODO: `licensing_info`에 기본 구현(default implementation)을 추가해서
    // 아래 두 struct 같은 구현체들이 함수를 반복하지 않고도
    // 기본 동작을 공유할 수 있게 해봐!
    // 기본 라이선스 정보는 "Default license" 문자열이어야 해.
    fn licensing_info(&self) -> String;
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // 이 줄은 수정하지 마!
impl Licensed for OtherSoftware {} // 이 줄은 수정하지 마!

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = "Default license";
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}

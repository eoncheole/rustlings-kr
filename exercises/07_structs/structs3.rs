// 구조체는 데이터를 담지만 로직도 가질 수 있어. 이 연습문제에는
// `Fireworks` 구조체와 이 구조체를 다루는 함수 몇 개가 정의되어 있어.
// 코드에서 이 관계가 더 명확히 드러나도록 독립 함수들을 메서드와
// 연관 함수로 바꿔봐.

#![deny(clippy::use_self)] // `Self` 타입을 사용하는 연습

#[derive(Debug)]
struct Fireworks {
    rockets: usize,
}

// TODO: 이 함수를 `Fireworks`의 연관 함수로 바꿔봐.
fn new_fireworks() -> Fireworks {
    Fireworks { rockets: 0 }
}

// TODO: 이 함수를 `Fireworks`의 메서드로 바꿔봐.
fn add_rockets(fireworks: &mut Fireworks, rockets: usize) {
    fireworks.rockets += rockets
}

// TODO: 이 함수를 `Fireworks`의 메서드로 바꿔봐.
fn start(fireworks: Fireworks) -> String {
    "🚀".repeat(fireworks.rockets)
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_some_fireworks() {
        let f = Fireworks::new();
        assert_eq!(f.start(), "");

        let mut f = Fireworks::new();
        f.add_rockets(3);
        assert_eq!(f.start(), "🚀🚀🚀");

        let mut f = Fireworks::new();
        f.add_rockets(7);
        // 마지막 테스트에서는 `start`가 `f`의 소유권을 가져가는지
        // 확인하기 위해 메서드 문법을 사용하지 않아.
        assert_eq!(Fireworks::start(f), "🚀🚀🚀🚀🚀🚀🚀");
    }
}

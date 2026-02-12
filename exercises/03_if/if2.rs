// TODO: 이 함수의 컴파일러 에러를 고쳐봐!
fn picky_eater(food: &str) -> &str {
    if food == "strawberry" {
        "Yummy!"
    } else {
        1
    }
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

// TODO: 테스트를 읽고 원하는 동작을 파악해봐.
// 테스트를 바꾸지 말고 모두 통과하게 만들어!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        // `picky_eater`에 "strawberry"를 인자로 넘기면 "Yummy!"를 반환해야 한다는 뜻이야.
        assert_eq!(picky_eater("strawberry"), "Yummy!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("potato"), "I guess I can eat that.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("broccoli"), "No thanks!");
        assert_eq!(picky_eater("gummy bears"), "No thanks!");
        assert_eq!(picky_eater("literally anything"), "No thanks!");
    }
}

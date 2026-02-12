fn trim_me(input: &str) -> &str {
    // TODO: 문자열 양쪽 끝의 공백(whitespace)을 제거해봐!
}

fn compose_me(input: &str) -> String {
    // TODO: 문자열에 " world!"를 붙여봐! 여러 가지 방법이 있어.
}

fn replace_me(input: &str) -> String {
    // TODO: 문자열에서 "cars"를 "balloons"로 바꿔봐!
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
        assert_eq!(trim_me("Hi!"), "Hi!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}

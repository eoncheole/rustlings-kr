// 이 연습문제에서는 반복자(Iterator)가 제공하는 고유한 장점들을 배워볼 거야.

// TODO: `capitalize_first` 함수를 완성해봐.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => todo!(),
    }
}

// TODO: `capitalize_first` 함수를 문자열 슬라이스(slice)의 슬라이스에 적용해봐.
// 문자열의 벡터(Vec)를 반환해야 해.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // ???
}

// TODO: `capitalize_first` 함수를 문자열 슬라이스의 슬라이스에 다시 적용해봐.
// 하나의 String을 반환해야 해.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    // ???
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}

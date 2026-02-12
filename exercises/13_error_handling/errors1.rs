// TODO: 이 함수는 빈 문자열을 넘기면 이름표에 출력할 텍스트를 생성하지 않아.
// 그냥 `None`을 반환하는 것보다 문제가 뭔지 설명해주면 더 좋겠지?
// 다행히 Rust에는 `Option`과 비슷하면서 에러 상황을 표현할 수 있는 구조가 있어.
// 함수 시그니처와 본문을 `Option<String>` 대신 `Result<String, String>`을
// 반환하도록 바꿔봐!
fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        // 빈 이름은 허용되지 않아
        None
    } else {
        Some(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Empty names aren't allowed"),
        );
    }
}

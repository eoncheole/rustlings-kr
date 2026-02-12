// 이건 다음 섹션들에 대한 퀴즈야:
// - 문자열(Strings)
// - 벡터(Vecs)
// - 이동 의미론(Move Semantics)
// - 모듈(Modules)
// - 열거형(Enums)
//
// 함수 형태의 작은 기계를 만들어보자! 입력으로 문자열과 명령어 목록을
// 줄 거야. 이 명령어들이 문자열에 어떤 동작을 적용할지 결정해.
// 가능한 동작은:
// - 문자열을 대문자로 변환
// - 문자열의 앞뒤 공백 제거
// - 문자열 뒤에 "bar"를 지정된 횟수만큼 추가
//
// 정확한 형태는 이래:
// - 입력은 길이가 2인 튜플(Tuple)의 Vec인데,
//   첫 번째 요소는 문자열이고 두 번째 요소는 명령어야.
// - 출력은 문자열의 벡터가 될 거야.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: 위에서 설명한 대로 함수를 완성해봐!
    // pub fn transformer(input: ???) -> ??? { ??? }
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    // TODO: `transformer`를 스코프에 가져오려면 뭘 임포트해야 할까?
    // use ???;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}

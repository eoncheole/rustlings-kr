fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: 이걸 값이 `Some`인 if-let 구문으로 만들어봐.
        word = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: 이걸 while-let 구문으로 만들어봐. `Vec::pop()`이 `Option`을
        // 한 겹 더 추가한다는 걸 기억해. if-let과 while-let 구문에서 중첩 패턴
        // 매칭(nested pattern matching)을 할 수 있어.
        integer = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}

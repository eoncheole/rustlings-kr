trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: String 벡터(Vec)에 대해 `AppendBar` 트레이트(Trait)를 구현해봐!
// `append_bar`는 벡터에 "Bar" 문자열을 push 해야 해.

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}

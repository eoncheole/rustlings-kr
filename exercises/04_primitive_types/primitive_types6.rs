fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: 튜플 인덱스를 사용해서 `numbers`의 두 번째 요소에 접근하고
        // `second`라는 변수에 할당해봐!
        // let second = ???;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}

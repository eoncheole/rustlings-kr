fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: 배열 `a`에서 `nice_slice`라는 슬라이스(Slice)를 만들어서 테스트를 통과하게 해봐!
        // let nice_slice = ???

        assert_eq!([2, 3, 4], nice_slice);
    }
}

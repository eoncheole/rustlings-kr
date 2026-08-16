fn elems_to_vec(a: i32, b: i32, c: i32) -> Vec<i32> {
    // TODO: 요소 `a`, `b`, `c`를 이 순서대로 담은 벡터를 반환해봐.
    // `vec!` 매크로를 사용해봐.
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elems_to_vec() {
        let (a, b, c) = (2, 7, 12);
        let v = elems_to_vec(a, b, c);
        assert_eq!(v[0], a);
        assert_eq!(v[1], b);
        assert_eq!(v[2], c);
    }
}

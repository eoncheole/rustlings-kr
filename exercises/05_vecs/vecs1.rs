fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // 배열

    // TODO: 배열 `a`와 똑같은 요소를 가진 `v`라는 벡터를 만들어봐!
    // 벡터 매크로를 사용해.
    // let v = ???;

    (a, v)
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: `vec0`과 `vec1` 두 벡터를 동시에 접근할 수 있도록 해서
    // 테스트의 컴파일러 에러를 고쳐봐!
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(vec0);

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}

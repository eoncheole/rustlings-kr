// TODO: 새로운 줄을 추가하지 않고 함수의 컴파일러 에러를 고쳐봐!
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}

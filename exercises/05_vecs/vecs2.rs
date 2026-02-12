fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        // TODO: `input` 슬라이스의 각 요소에 2를 곱해서 `output` 벡터에 push해봐!
    }

    output
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}

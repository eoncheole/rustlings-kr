fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    // TODO: 테스트 안의 줄 순서만 바꿔서 컴파일러 에러를 고쳐봐!
    // 줄을 추가하거나, 변경하거나, 제거하지 마.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        let z = &mut x;
        y.push(42);
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}

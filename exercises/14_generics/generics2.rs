// 이 강력한 래퍼(wrapper)는 양의 정수 값을 저장하는 기능을 제공해.
// TODO: 제네릭(Generic)을 사용해서 어떤 타입이든 감쌀 수 있도록 다시 작성해봐!
struct Wrapper {
    value: u32,
}

// TODO: struct의 구현을 감싸는 값에 대해 제네릭하게 수정해봐!
impl Wrapper {
    fn new(value: u32) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}

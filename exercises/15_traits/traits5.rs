trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct;
impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

struct OtherStruct;
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// TODO: 이 함수의 시그니처만 변경해서 컴파일러 에러를 고쳐봐!
fn some_func(item: ???) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_func() {
        assert!(some_func(SomeStruct));
        assert!(some_func(OtherStruct));
    }
}

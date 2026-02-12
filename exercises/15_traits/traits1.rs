// `AppendBar` 트레이트(Trait)는 이 트레이트를 구현하는 모든 객체에
// "Bar"를 추가하는 함수 하나만 가지고 있어.
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: `String` 타입에 대해 `AppendBar`를 구현해봐!
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), "FooBar");
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(String::from("").append_bar().append_bar(), "BarBar");
    }
}

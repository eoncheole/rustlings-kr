# 트레이트(Trait)

트레이트(Trait)는 메서드의 모음이야.

데이터 타입은 트레이트를 구현할 수 있어. 그러려면 트레이트를 구성하는 메서드들을 해당 데이터 타입에 대해 정의하면 돼. 예를 들어, `String` 데이터 타입은 `From<&str>` 트레이트를 구현하고 있어. 덕분에 `String::from("hello")`처럼 쓸 수 있는 거지.

이런 면에서 트레이트는 Java의 인터페이스(Interface)나 C++의 추상 클래스(Abstract Class)와 비슷해.

자주 쓰이는 Rust 트레이트 몇 가지를 소개할게:

- `Clone` (`clone` 메서드)
- `Display` (`{}`를 통한 포맷 출력을 가능하게 해줘)
- `Debug` (`{:?}`를 통한 포맷 출력을 가능하게 해줘)

트레이트는 데이터 타입 간의 공유 동작을 나타내기 때문에, 제네릭(Generic)을 작성할 때 유용해.

## 더 알아보기

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)

# 타입 변환(Type Conversions)

Rust는 주어진 타입의 값을 다른 타입으로 변환하는 다양한 방법을 제공해.

가장 간단한 형태의 타입 변환은 타입 캐스트 표현식이야. 이진 연산자 `as`로 표시돼. 예를 들어, `println!("{}", 1 + 1.0);`은 `1`이 정수이고 `1.0`이 부동소수점이라서 컴파일되지 않아. 하지만 `println!("{}", 1 as f32 + 1.0)`은 컴파일돼. [`conversions1`](conversions1.rs) 연습문제에서 이걸 다뤄.

Rust는 또한 구현 시 타입 변환을 쉽게 해주는 트레이트(Trait)들을 제공해. 이 트레이트들은 [`convert`](https://doc.rust-lang.org/std/convert/index.html) 모듈에서 찾을 수 있어.
해당 트레이트들은 다음과 같아:

- `From`과 `Into`는 [`conversions2`](conversions2.rs)에서 다뤄
- `TryFrom`과 `TryInto`는 [`conversions4`](conversions4.rs)에서 다뤄
- `AsRef`와 `AsMut`는 [`conversions5`](conversions5.rs)에서 다뤄

추가로, `std::str` 모듈은 [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html)이라는 트레이트를 제공하는데, 이건 문자열의 `parse` 메서드를 통해 문자열을 대상 타입으로 변환하는 데 도움을 줘. 주어진 타입 `Person`에 대해 올바르게 구현되면, `let p: Person = "Mark,20".parse().unwrap()`이 컴파일되고 패닉 없이 실행돼야 해.

이것들이 ***표준 라이브러리 내에서*** 데이터를 원하는 타입으로 변환하는 주요 방법이야.

## 더 알아보기

이것들은 책에서 직접적으로 다루지는 않지만, 표준 라이브러리에 훌륭한 문서가 있어.

- [conversions](https://doc.rust-lang.org/std/convert/index.html)
- [`FromStr` trait](https://doc.rust-lang.org/std/str/trait.FromStr.html)

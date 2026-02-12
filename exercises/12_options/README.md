# Options

Option 타입은 선택적 값(optional value)을 나타내: 모든 Option은 값을 담고 있는 Some이거나, 값이 없는 None이야.
Option 타입은 Rust 코드에서 정말 많이 쓰이는데, 다양한 용도가 있어:

- 초깃값(initial values)
- 입력 범위 전체에 대해 정의되지 않은 함수의 반환값 (부분 함수(partial functions))
- 단순한 에러를 보고할 때의 반환값, 에러 시 None을 반환
- 선택적 struct 필드(optional struct fields)
- 빌려주거나 "가져갈" 수 있는 struct 필드
- 선택적 함수 인자(optional function arguments)
- 널 가능 포인터(nullable pointers)
- 까다로운 상황에서 값을 교체(swap)할 때

## 더 알아보기

- [Option 열거형(Enum) 형식](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)
- [Option 모듈 문서](https://doc.rust-lang.org/std/option/)
- [Option 열거형(Enum) 문서](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)

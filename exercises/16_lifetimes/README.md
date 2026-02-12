# 수명(Lifetime)

수명(Lifetime)은 컴파일러에게 참조(reference)가 어떤 상황에서든 유효할 만큼
충분히 오래 살아있는지 검사하는 방법을 알려줘. 예를 들어, 수명은
"매개변수 'a'가 매개변수 'b'만큼 오래 살아있어야 반환값이 유효하다"라고 말하는 거야.

수명은 빌림(borrow), 즉 참조에서만 필요해.
복사된 매개변수나 이동(move)된 값은 자기 스코프 안에서 소유되기 때문에
바깥에서 참조할 수 없거든. 수명 덕분에 함수 같은 걸 호출하는 코드에서
인자가 유효한지 검사할 수 있어. 수명은 호출자에게 제약을 거는 거야.

수명 어노테이션(lifetime annotation)에 대해 더 알고 싶다면,
[lifetimekata](https://tfpk.github.io/lifetimekata/) 프로젝트를 확인해봐.
Rustlings와 비슷한 스타일의 연습 문제인데, 수명 어노테이션 작성법을
배우는 데 집중하는 프로젝트야.

## 더 알아보기

- [Lifetimes (in Rust By Example)](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)

# Vectors

Vec는 Rust에서 가장 많이 사용되는 데이터 구조 중 하나야. 다른 프로그래밍 언어에서는 단순히 배열이라고 부르지만, Rust는 좀 더 저수준에서 동작하기 때문에 배열(Array)은 스택에 저장되고 (크기를 늘리거나 줄일 수 없고, 컴파일 타임에 크기를 알아야 해), Vec는 힙에 저장돼 (이런 제약이 없어).

Vec는 책에서는 좀 뒤에 나오는 챕터지만, 충분히 유용하니까 좀 더 일찍 다뤄보려고 해. 또 다른 유용한 데이터 구조인 해시맵(HashMap)은 나중에 다룰 거야.

## 더 알아보기

- [Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [`iter_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)

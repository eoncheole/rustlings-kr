// 컴파일 시점에 Rust는 타입이 얼마나 많은 공간을 차지하는지 알아야 해. 이게
// 재귀 타입(recursive type)에서 문제가 되는데, 값의 일부로 같은 타입의 다른
// 값을 가질 수 있거든. 이 문제를 해결하려면 `Box`를 쓰면 돼 - 힙(heap)에
// 데이터를 저장하는 스마트 포인터(Smart Pointer)로, 재귀 타입을 감쌀 수 있게
// 해줘.
//
// 이 연습문제에서 구현할 재귀 타입은 "콘스 리스트(cons list)"야. 함수형
// 프로그래밍 언어에서 자주 볼 수 있는 자료구조지. 콘스 리스트의 각 항목은
// 두 가지 요소를 갖고 있어: 현재 항목의 값과 다음 항목. 마지막 항목은 `Nil`
// 이라는 값이야.

// TODO: enum 정의에서 `Box`를 사용해서 코드가 컴파일되게 만들어봐.
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, List),
    Nil,
}

// TODO: 빈 콘스 리스트를 만들어봐.
fn create_empty_list() -> List {
    todo!()
}

// TODO: 비어있지 않은 콘스 리스트를 만들어봐.
fn create_non_empty_list() -> List {
    todo!()
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}

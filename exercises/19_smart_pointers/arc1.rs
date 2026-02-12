// 이 연습문제에서는 0부터 99까지의 값을 가진 `numbers`라는 `Vec<u32>`가 주어져.
// 이 숫자들을 8개의 서로 다른 스레드(thread)에서 동시에 사용하고 싶어. 각
// 스레드는 오프셋을 기준으로 8번째마다 오는 값들의 합을 구할 거야.
//
// 첫 번째 스레드 (오프셋 0)는 0, 8, 16, …을 합산해.
// 두 번째 스레드 (오프셋 1)는 1, 9, 17, …을 합산해.
// 세 번째 스레드 (오프셋 2)는 2, 10, 18, …을 합산해.
// …
// 여덟 번째 스레드 (오프셋 7)는 7, 15, 23, …을 합산해.
//
// 각 스레드는 숫자 벡터에 대한 참조 카운팅 포인터(reference-counting pointer)를
// 소유해야 해. 하지만 `Rc`는 스레드 안전(thread-safe)하지 않아. 그래서 `Arc`를
// 써야 해.
//
// 스레드를 생성하고 조인하는 방법에 신경 쓰지 마. 그건 나중에 스레드 관련
// 연습문제에서 연습할 거야.

// 아래 줄들은 변경하지 마.
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // TODO: `Arc`를 사용해서 `shared_numbers`를 정의해봐.
    // let shared_numbers = ???;

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // TODO: `shared_numbers`를 사용해서 `child_numbers`를 정의해봐.
        // let child_numbers = ???;

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}

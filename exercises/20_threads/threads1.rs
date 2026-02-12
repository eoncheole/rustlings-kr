// 이 프로그램은 각각 최소 250ms 동안 실행되는 여러 스레드(Thread)를 생성하고,
// 각 스레드는 완료하는 데 걸린 시간을 반환해. 프로그램은 생성된 모든 스레드가
// 끝날 때까지 기다리고, 반환값들을 벡터(Vec)에 모아야 해.

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} done");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        // TODO: 모든 스레드의 결과를 `results` 벡터에 모아봐.
        // `thread::spawn`이 반환하는 `JoinHandle` 구조체를 사용해봐.
    }

    if results.len() != 10 {
        panic!("Oh no! Some thread isn't done yet!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} took {result}ms");
    }
}

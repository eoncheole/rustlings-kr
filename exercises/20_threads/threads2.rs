// 이전 연습문제에 이어서, 모든 스레드가 작업을 완료하길 원해.
// 하지만 이번에는 생성된 스레드들이 공유 값인 `JobStatus.jobs_done`을
// 직접 업데이트해야 해.

use std::{sync::Arc, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: **가변(mutable)** 공유 상태를 원한다면 `Arc`만으로는 부족해.
    let status = Arc::new(JobStatus { jobs_done: 0 });

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: 공유 값을 업데이트하기 전에 먼저 해야 할 작업이 있어.
            status_shared.jobs_done += 1;
        });
        handles.push(handle);
    }

    // 모든 작업이 완료될 때까지 기다리는 중.
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: `JobStatus.jobs_done`의 값을 출력해봐.
    println!("Jobs done: {}", todo!());
}

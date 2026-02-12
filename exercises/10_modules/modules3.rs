// `use` 키워드를 사용하면 어디서든, 특히 표준 라이브러리(Standard Library)에서
// 모듈 경로를 네 스코프로 가져올 수 있어!

// TODO: `std::time` 모듈에서 `SystemTime`과 `UNIX_EPOCH`를 네 스코프로
// 가져와봐! 한 줄로 할 수 있으면 보너스 포인트야!
// use ???;

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

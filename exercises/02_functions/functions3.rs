fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: 함수 호출을 고쳐봐!
    call_me();
}

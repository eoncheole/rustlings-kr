// TODO: 콜론 `:` 뒤에 인자 `num`의 빠진 타입을 추가해봐!
fn call_me(num:) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}

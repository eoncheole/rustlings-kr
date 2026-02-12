// 이 가게는 세일 중인데, 가격이 짝수면 10 Rustbucks 할인, 홀수면 3 Rustbucks 할인이야.
// 함수 본문은 신경 쓰지 마. 지금은 함수 시그니처에만 집중하면 돼.

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO: 함수 시그니처를 고쳐봐!
fn sale_price(price: i64) -> {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

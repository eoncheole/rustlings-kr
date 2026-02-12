// 이 프로그램은 이전 연습문제에서 완성한 `total_cost` 함수를 사용하려고 해.
// 그런데 동작하지 않아! 왜 그럴까? 어떻게 고쳐야 할까?

use std::num::ParseIntError;

// 이 함수는 수정하지 마.
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// TODO: `main` 함수의 시그니처와 본문을 바꿔서 컴파일러 에러를 고쳐봐!
fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // 이 줄은 수정하지 마.
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {tokens} tokens.");
    }
}

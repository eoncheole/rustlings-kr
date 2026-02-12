// 토큰으로 아이템을 살 수 있는 게임을 만들고 있다고 해보자. 모든 아이템은
// 5토큰이고, 아이템을 구매할 때마다 1토큰의 수수료가 붙어.
// 플레이어가 사고 싶은 아이템 수를 입력하면, `total_cost` 함수가
// 총 비용을 계산해줘. 플레이어가 수량을 직접 입력하니까 문자열로 받게 되는데,
// 숫자가 아닌 아무거나 입력했을 수도 있어!
//
// 지금은 이 함수가 에러 케이스를 전혀 처리하지 않고 있어. 우리가 원하는 건:
// 숫자가 아닌 문자열로 `total_cost` 함수를 호출하면 `ParseIntError`를
// 반환하는 거야. 그 경우에는 곱셈이나 덧셈을 시도하지 않고 바로 에러를
// 반환해야 해.
//
// 올바른 구현 방법이 최소 두 가지는 있어. 그중 하나가 훨씬 짧지!

use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // TODO: 위에서 설명한 대로 에러 케이스를 처리해봐.
    let qty = item_quantity.parse::<i32>();

    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}

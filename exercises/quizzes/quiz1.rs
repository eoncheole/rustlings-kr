// 이건 다음 섹션들에 대한 퀴즈야:
// - 변수(Variables)
// - 함수(Functions)
// - If
//
// Mary가 사과를 사고 있어. 사과 가격은 다음과 같이 계산돼:
// - 사과 한 개의 가격은 2 rustbuck이야.
// - 하지만 Mary가 40개를 초과해서 사면, 전체 주문에서 사과 한 개당 가격이
// 1 rustbuck으로 할인돼!

// TODO: 구매 수량이 주어졌을 때 사과 주문 가격을 계산하는 함수를 작성해봐!
// fn calculate_price_of_apples(???) -> ??? { ??? }

fn main() {
    // 여기서 자유롭게 실험해봐.
}

// 테스트는 바꾸지 마!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}

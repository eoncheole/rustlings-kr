// 해시맵(HashMap) 형태로 과일 바구니를 정의해야 해. 키(key)는 과일 이름을
// 나타내고, 값(value)은 바구니에 있는 해당 과일의 개수를 나타내. 최소 3종류
// 이상의 과일(예: apple, banana, mango)을 바구니에 넣어야 하고, 전체 과일
// 개수의 합은 최소 5개 이상이어야 해.

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: 해시맵을 선언해봐.
    // let mut basket =

    // 바나나 2개는 이미 넣어뒀어 :)
    basket.insert(String::from("banana"), 2);

    // TODO: 바구니에 과일을 더 넣어봐.

    basket
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}

// 맛있는 과일 케이크를 만들기 위해 다양한 과일을 모으고 있어. 이를 위해
// 해시맵(HashMap) 형태의 바구니가 있어. 키(key)는 수집한 각 과일의 이름을
// 나타내고, 값(value)은 해당 과일을 몇 개 모았는지를 나타내. 이미 바구니
// 해시맵에는 Apple(4개), Mango(2개), Lychee(5개) 세 종류의 과일이 들어있어.
// 바구니에 과일을 추가해서 각 종류마다 최소 1개 이상, 전체 합이 11개를 초과
// 하도록 만들어야 해 - 먹여야 할 입이 많거든. 이미 바구니에 있는 과일(Apple,
// Mango, Lychee)은 더 넣으면 안 돼.

use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = [
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        // TODO: 바구니에 아직 없는 새로운 과일을 추가해봐. 이미 들어있는
        // 종류의 과일은 넣으면 안 된다는 걸 기억해!
    }
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    // 이 함수는 수정하지 마!
    fn get_fruit_basket() -> HashMap<Fruit, u32> {
        let content = [(Fruit::Apple, 4), (Fruit::Mango, 2), (Fruit::Lychee, 5)];
        HashMap::from_iter(content)
    }

    #[test]
    fn test_given_fruits_are_not_modified() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
        assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
    }

    #[test]
    fn at_least_five_types_of_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count_fruit_kinds = basket.len();
        assert!(count_fruit_kinds >= 5);
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count = basket.values().sum::<u32>();
        assert!(count > 11);
    }

    #[test]
    fn all_fruit_types_in_basket() {
        let fruit_kinds = [
            Fruit::Apple,
            Fruit::Banana,
            Fruit::Mango,
            Fruit::Lychee,
            Fruit::Pineapple,
        ];

        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);

        for fruit_kind in fruit_kinds {
            let Some(amount) = basket.get(&fruit_kind) else {
                panic!("Fruit kind {fruit_kind:?} was not found in basket");
            };
            assert!(*amount > 0);
        }
    }
}

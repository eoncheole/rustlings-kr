// `use`와 `as` 키워드를 사용해서 모듈 경로(Module path)를 스코프(Scope)로
// 가져오고 새 이름을 붙일 수 있어!

mod delicious_snacks {
    // TODO: 아래 두 `use` 구문을 수정한 다음 추가해봐!
    // use self::fruits::PEAR as ???;
    // use self::veggies::CUCUMBER as ???;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}

// TODO: 비공개(private) 함수를 호출해서 생기는 컴파일러 에러를 고쳐봐!
mod sausage_factory {
    // 이 모듈 바깥에서는 아무도 이걸 볼 수 없게 해야 해!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}

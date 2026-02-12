#![allow(clippy::ptr_arg)]

// TODO: 참조(`&` 문자)를 추가하거나 제거하는 것만으로 컴파일러 에러를 고쳐봐!

// 소유권을 가져가면 안 됨
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// 소유권을 가져가야 함
fn string_uppercase(mut data: &String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data);

    string_uppercase(&data);
}

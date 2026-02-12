// 이 함수 호출을 `string_slice` 또는 `string` 호출로 바꿔야 해!
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: 여기 여러 값들이 있어 - 어떤 건 `String`이고, 어떤 건 `&str`이야.
// `placeholder(…)`를 각 값의 타입에 따라 `string_slice(…)` 또는
// `string(…)`으로 바꿔봐!
fn main() {
    placeholder("blue");

    placeholder("red".to_string());

    placeholder(String::from("hi"));

    placeholder("rust is fun!".to_owned());

    placeholder("nice weather".into());

    placeholder(format!("Interpolation {}", "Station"));

    // 주의: 이건 바이트 인덱싱(byte indexing)이지, 문자 인덱싱(character indexing)이 아니야.
    // 문자 인덱싱은 `s.chars().nth(INDEX)`를 사용하면 돼.
    placeholder(&String::from("abc")[0..1]);

    placeholder("  hello there ".trim());

    placeholder("Happy Monday!".replace("Mon", "Tues"));

    placeholder("mY sHiFt KeY iS sTiCkY".to_lowercase());
}

fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Clippy 린트(Lint)를 고쳐봐!
    for x in option {
        res += x;
    }

    println!("{res}");
}

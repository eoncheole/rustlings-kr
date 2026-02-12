// Clippy의 유용함을 알 수 있는 쉬운 수정 몇 가지를 더 준비했어.
// TODO: 모든 Clippy 린트(Lint)를 고쳐봐!

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // `my_option`의 값을 모른다고 가정해봐.
    // `Some`인 경우에 그 값을 출력하고 싶어.
    if my_option.is_none() {
        println!("{}", my_option.unwrap());
    }

    #[rustfmt::skip]
    let my_arr = &[
        -1, -2, -3
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    let mut my_vec = vec![1, 2, 3, 4, 5];
    my_vec.resize(0, 5);
    println!("This Vec is empty, see? {my_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // 이 둘을 바꿔보자!
    value_a = value_b;
    value_b = value_a;
    println!("value a: {value_a}; value b: {value_b}");
}

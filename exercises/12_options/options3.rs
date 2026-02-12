#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: 이 match 구문에 뭔가를 추가해서 컴파일러 에러를 고쳐봐.
    match optional_point {
        Some(p) => println!("Coordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }

    println!("{optional_point:?}"); // 이 줄은 바꾸지 마.
}

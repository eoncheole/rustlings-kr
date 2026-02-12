// Rust에서 타입 캐스팅(Type Casting)은 `as` 연산자를 사용해서 해.
// `as` 연산자는 타입 캐스팅뿐만 아니라 임포트 이름 변경에도 사용된다는 걸
// 알아둬!

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    // TODO: 나누기 전에 변환을 해봐!
    total / values.len()
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}

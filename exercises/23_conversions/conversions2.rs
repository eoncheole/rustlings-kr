// `From` 트레이트는 값을 다른 타입의 값으로 변환할 때 사용해. `From`을
// 구현하면 `Into` 구현도 자동으로 제공돼. 자세한 내용은 문서를 읽어봐:
// https://doc.rust-lang.org/std/convert/trait.From.html
//
// 측정 단위마다 별도 타입을 사용하는 것은 일반적인 방식이야. 이렇게 하면
// 단위가 다른 값을 실수로 섞는 일을 막을 수 있어.

struct Celsius(f64);

struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    // TODO: 섭씨를 화씨로 변환해봐. 부동 소수점 정밀도는 신경 쓰지
    // 않아도 돼. 공식은 F = C * 1.8 + 32야.
}

impl From<Fahrenheit> for Celsius {
    // TODO: 화씨를 섭씨로 변환해봐.
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: [(f64, f64); 6] = [
        (-50.0, -58.0),
        (0.0, 32.0),
        (20.0, 68.0),
        (100.0, 212.0),
        (400.0, 752.0),
        (1000.0, 1832.0),
    ];

    #[test]
    fn celsius_to_fahrenheit() {
        for (celsius, fahrenheit) in CASES {
            let Fahrenheit(actual) = Celsius(celsius).into();
            assert_eq!(actual.round(), fahrenheit);
        }
    }

    #[test]
    fn fahrenheit_to_celsius() {
        for (celsius, fahrenheit) in CASES {
            let Celsius(actual) = Fahrenheit(fahrenheit).into();
            assert_eq!(actual.round(), celsius);
        }
    }
}

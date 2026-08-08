// 이전 `from_into` 연습문제와 비슷해. 하지만 이번에는 기본값으로 대체하는 대신
// `FromStr`을 구현하고 에러를 반환할 거야. 추가로, `FromStr`을 구현하면
// 문자열의 `parse` 메서드를 사용해서 해당 타입의 객체를 생성할 수 있어.
// 자세한 내용은 문서를 읽어봐:
// https://doc.rust-lang.org/std/str/trait.FromStr.html

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

// `FromStr` 구현에서 이 에러 타입을 사용할 거야.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // 필드 수가 올바르지 않음
    BadLen,
    // 이름 필드가 비어있음
    NoName,
    // parse::<u8>()에서 발생한 래핑된 에러
    ParseInt(ParseIntError),
}

// TODO: "Mark,20" 형태의 문자열에서 `Person`을 파싱할 수 있도록
// 이 `FromStr` 구현을 완성해봐!
// 나이 부분을 `u8`로 파싱해야 하는데, `"4".parse::<u8>()` 같은 걸 쓰면 돼.
//
// 단계:
// 1. 주어진 문자열을 쉼표로 분리해.
// 2. 분리 결과가 2개보다 적거나 많으면
//    `ParsePersonError::BadLen` 에러를 반환해.
// 3. 분리 결과의 첫 번째 요소를 이름으로 사용해.
// 4. 이름이 비어있으면 `ParsePersonError::NoName` 에러를 반환해.
// 5. 분리 결과의 두 번째 요소를 `u8`로 파싱해서 나이로 사용해.
// 6. 나이 파싱에 실패하면 `ParsePersonError::ParseInt` 에러를 반환해.
impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {}
}

fn main() {
    let p = "Mark,20".parse::<Person>();
    println!("{p:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ParsePersonError::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!("John,".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!("John,twenty".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(",".parse::<Person>(), Err(NoName | ParseInt(_))));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(NoName | ParseInt(_)),
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!("John,32,man".parse::<Person>(), Err(BadLen));
    }
}

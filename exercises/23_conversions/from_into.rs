// `From` 트레이트(Trait)는 값-대-값 변환(Conversion)에 사용돼. `From`을
// 구현하면 `Into` 구현이 자동으로 제공돼.
// 자세한 내용은 문서를 읽어봐:
// https://doc.rust-lang.org/std/convert/trait.From.html

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 제공된 문자열이 `Person` 객체로 변환할 수 없을 때 대체값으로 사용하기 위해
// Default 트레이트를 구현해.
impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from("John"),
            age: 30,
        }
    }
}

// TODO: "Mark,20" 형태의 문자열에서 `Person`을 파싱할 수 있도록
// 이 `From` 구현을 완성해봐!
// 나이 부분을 `u8`로 파싱해야 하는데, `"4".parse::<u8>()` 같은 걸 쓰면 돼.
//
// 단계:
// 1. 주어진 문자열을 쉼표로 분리해.
// 2. 분리 결과가 2개보다 적거나 많으면 `Person`의 기본값을 반환해.
// 3. 분리 결과의 첫 번째 요소를 이름으로 사용해.
// 4. 이름이 비어있으면 `Person`의 기본값을 반환해.
// 5. 분리 결과의 두 번째 요소를 `u8`로 파싱해서 나이로 사용해.
// 6. 나이 파싱에 실패하면 `Person`의 기본값을 반환해.
impl From<&str> for Person {
    fn from(s: &str) -> Self {}
}

fn main() {
    // `from` 함수를 사용해.
    let p1 = Person::from("Mark,20");
    println!("{p1:?}");

    // `From`이 Person에 구현되어 있으니까 `Into`도 사용할 수 있어.
    let p2: Person = "Gerald,70".into();
    println!("{p2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }

    #[test]
    fn test_bad_convert() {
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_good_convert() {
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }

    #[test]
    fn test_bad_age() {
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,dog");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}

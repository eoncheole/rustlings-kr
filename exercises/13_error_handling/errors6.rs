// `Box<dyn Error>` 같은 포괄적인 에러 타입은 라이브러리 코드에서는
// 권장되지 않아. 호출자가 에러를 출력하거나 전파하는 대신 에러 내용에 따라
// 결정을 내리고 싶을 수도 있거든. 여기서는 커스텀 에러 타입(Custom Error Type)을
// 정의해서, 함수가 에러를 반환했을 때 호출자가 다음에 뭘 할지 결정할 수 있게 해보자.

use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// `PositiveNonzeroInteger::parse`에서 사용할 커스텀 에러 타입이야.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> Self {
        Self::Creation(err)
    }

    // TODO: 여기에 또 다른 에러 변환 함수를 추가해봐.
    // fn from_parse_int(???) -> Self { ??? }
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(Self(x as u64)),
        }
    }

    fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {
        // TODO: `parse()`가 에러를 반환할 때 패닉하는 대신 적절한 에러를
        // 반환하도록 바꿔봐.
        let x: i64 = s.parse().unwrap();
        Self::new(x).map_err(ParsePosNonzeroError::from_creation)
    }
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            PositiveNonzeroInteger::parse("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_)),
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            PositiveNonzeroInteger::parse("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative)),
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            PositiveNonzeroInteger::parse("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero)),
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42).unwrap();
        assert_eq!(x.0, 42);
        assert_eq!(PositiveNonzeroInteger::parse("42"), Ok(x));
    }
}

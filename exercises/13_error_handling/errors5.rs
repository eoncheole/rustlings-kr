// 이 연습문제는 `errors4`의 변형 버전이야. `Box`와 `From` 트레이트(trait)처럼
// 나중에 배울 개념들을 사용하고 있어. 지금 당장 자세히 이해할 필요는 없지만,
// 미리 읽어보고 싶으면 그래도 돼. 일단 `Box<dyn ???>` 타입은
// "???를 구현하는 아무거나 다 됨" 타입이라고 생각하면 돼.
//
// 간단히 말하면, Box의 이런 사용법은 값을 소유하면서 특정 트레이트를
// 구현하는 타입이기만 하면 되는 경우에 쓰여. 이렇게 하려면 `Box`를
// `Box<dyn Trait>` 타입으로 선언하는데, 여기서 `Trait`는 컴파일러가
// 해당 컨텍스트에서 사용되는 값에 대해 찾는 트레이트야.
// 이 연습문제에서 그 컨텍스트는 `Result`에서 반환될 수 있는 에러들이야.

use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// `CreationError`가 `Error`를 구현하려면 이게 필요해.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// TODO: 올바른 반환 타입 `Result<(), Box<dyn ???>>`을 추가해봐. 두 가지 에러를
// 모두 나타내려면 뭘 쓸 수 있을까? 두 에러가 공통으로 구현하는 트레이트가 있을까?
fn main() {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

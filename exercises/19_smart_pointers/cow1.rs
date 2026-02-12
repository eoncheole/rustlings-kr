// 이 연습문제는 `Cow` (Clone-On-Write) 스마트 포인터를 탐구해볼 거야. `Cow`는
// 빌린 데이터(borrowed data)를 감싸서 불변 접근을 제공하고, 변경이나 소유권이
// 필요할 때 지연 복제(lazy clone)를 해. 이 타입은 `Borrow` 트레이트(trait)를
// 통해 일반적인 빌린 데이터와 함께 작동하도록 설계됐어.

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // 아직 소유하고 있지 않다면 벡터로 복제해.
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() {
        // `input`이 변경되어야 하기 때문에 복제가 발생해.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        // `input`이 변경될 필요가 없어서 복제가 발생하지 않아.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        // TODO: `todo!()`를 `Cow::Owned(_)` 또는 `Cow::Borrowed(_)`로 바꿔봐.
        assert!(matches!(input, todo!()));
    }

    #[test]
    fn owned_no_mutation() {
        // `vec`을 `&` 없이 전달해서 `Cow`가 직접 소유하게 할 수도 있어. 이
        // 경우에는 변경이 발생하지 않고 (모든 숫자가 이미 절댓값이니까) 따라서
        // 복제도 없어. 하지만 결과는 여전히 소유된 상태야, 왜냐하면 빌리거나
        // 변경된 적이 없기 때문이야.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: `todo!()`를 `Cow::Owned(_)` 또는 `Cow::Borrowed(_)`로 바꿔봐.
        assert!(matches!(input, todo!()));
    }

    #[test]
    fn owned_mutation() {
        // 물론 변경이 발생하는 경우에도 마찬가지야 (모든 숫자가 절댓값은
        // 아닌 경우). 이 경우 `abs_all` 함수에서 `to_mut()` 호출은 이전과
        // 같은 데이터에 대한 참조를 반환해.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: `todo!()`를 `Cow::Owned(_)` 또는 `Cow::Borrowed(_)`로 바꿔봐.
        assert!(matches!(input, todo!()));
    }
}

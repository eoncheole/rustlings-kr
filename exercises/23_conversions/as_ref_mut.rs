// AsRef와 AsMut는 저렴한 참조-대-참조 변환(Conversion)을 가능하게 해줘. 자세한 내용은
// https://doc.rust-lang.org/std/convert/trait.AsRef.html 과
// https://doc.rust-lang.org/std/convert/trait.AsMut.html 을 읽어봐.

// 주어진 인자의 바이트 수(문자 수가 아님)를 구하는 함수야
// (`.len()`은 문자열의 바이트 수를 반환해).
// TODO: `AsRef` 트레이트(Trait)를 트레이트 바운드로 적절하게 추가해봐!
fn byte_counter<T>(arg: T) -> usize {
    arg.as_ref().len()
}

// 주어진 인자의 문자 수(바이트 수가 아님)를 구하는 함수야.
// TODO: `AsRef` 트레이트를 트레이트 바운드로 적절하게 추가해봐!
fn char_counter<T>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// `as_mut()`를 사용해서 숫자를 제곱하는 함수야.
// TODO: 적절한 트레이트 바운드를 추가해봐!
fn num_sq<T>(arg: &mut T) {
    // TODO: 함수 본문을 구현해봐!
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}

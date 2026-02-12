struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // 이 함수는 수정하지 마.
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // 여기서는 Result를 반환하는 게 더 좋겠지만, 패닉(panic)을 일으킬 수
            // 있는 함수를 테스트하는 방법을 배워보자.
            panic!("Rectangle width and height must be positive");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // TODO: 이 테스트는 생성자(constructor)에 전달한 크기대로 직사각형이
        // 만들어졌는지 확인해야 해.
        let rect = Rectangle::new(10, 20);
        assert_eq!(todo!(), 10); // 너비 확인
        assert_eq!(todo!(), 20); // 높이 확인
    }

    // TODO: 이 테스트는 음수 너비로 직사각형을 만들려고 할 때 프로그램이
    // 패닉(panic)을 일으키는지 확인해야 해.
    #[test]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    // TODO: 이 테스트는 음수 높이로 직사각형을 만들려고 할 때 프로그램이
    // 패닉을 일으키는지 확인해야 해.
    #[test]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}

// 이 연습문제에서는 `Rc<T>` 타입을 통해 다중 소유권(multiple ownership)의
// 개념을 표현해보자. 이건 태양계 모델이야 - `Sun` 타입이 하나 있고 여러 개의
// `Planet`이 있어. 행성들이 태양의 소유권을 가져가는데, 이건 행성들이 태양
// 주위를 공전한다는 걸 나타내.

use std::rc::Rc;

#[derive(Debug)]
struct Sun;

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {self:?}!");
    }
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc1() {
        let sun = Rc::new(Sun);
        println!("reference count = {}", Rc::strong_count(&sun)); // 참조 1개

        let mercury = Planet::Mercury(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 참조 2개
        mercury.details();

        let venus = Planet::Venus(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 참조 3개
        venus.details();

        let earth = Planet::Earth(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 참조 4개
        earth.details();

        let mars = Planet::Mars(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 참조 5개
        mars.details();

        let jupiter = Planet::Jupiter(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 참조 6개
        jupiter.details();

        // TODO
        let saturn = Planet::Saturn(Rc::new(Sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 참조 7개
        saturn.details();

        // TODO
        let uranus = Planet::Uranus(Rc::new(Sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 참조 8개
        uranus.details();

        // TODO
        let neptune = Planet::Neptune(Rc::new(Sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 참조 9개
        neptune.details();

        assert_eq!(Rc::strong_count(&sun), 9);

        drop(neptune);
        println!("reference count = {}", Rc::strong_count(&sun)); // 참조 8개

        drop(uranus);
        println!("reference count = {}", Rc::strong_count(&sun)); // 참조 7개

        drop(saturn);
        println!("reference count = {}", Rc::strong_count(&sun)); // 참조 6개

        drop(jupiter);
        println!("reference count = {}", Rc::strong_count(&sun)); // 참조 5개

        drop(mars);
        println!("reference count = {}", Rc::strong_count(&sun)); // 참조 4개

        // TODO
        println!("reference count = {}", Rc::strong_count(&sun)); // 참조 3개

        // TODO
        println!("reference count = {}", Rc::strong_count(&sun)); // 참조 2개

        // TODO
        println!("reference count = {}", Rc::strong_count(&sun)); // 참조 1개

        assert_eq!(Rc::strong_count(&sun), 1);
    }
}

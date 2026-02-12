struct ColorRegularStruct {
    // TODO: `regular_structs` 테스트가 기대하는 필드들을 추가해봐!
    // 필드의 타입은 뭐여야 할까? RGB 색상의 최솟값과 최댓값은?
}

struct ColorTupleStruct(/* TODO: `tuple_structs` 테스트가 기대하는 필드들을 추가해봐! */);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: 일반 구조체를 인스턴스화해봐!
        // let green =

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: 튜플 구조체를 인스턴스화해봐!
        // let green =

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: 유닛 구조체를 인스턴스화해봐!
        // let unit_struct =
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}

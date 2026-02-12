// 이 퀴즈는 다음을 테스트해:
// - 제네릭(Generics)
// - 트레이트(Traits)
//
// 가상의 마법 학교에 Rust로 작성된 새로운 성적표 생성 시스템이 생겼어!
// 현재 시스템은 학생의 성적을 숫자로 표현하는 성적표만 지원해 (예: 1.0 -> 5.5).
// 하지만 학교에서는 알파벳 성적(A+ -> F-)도 발급하기 때문에 두 가지 유형의
// 성적표를 모두 출력할 수 있어야 해!
//
// `ReportCard` struct와 impl 블록에서 숫자 성적표 외에 알파벳 성적표도
// 지원하도록 필요한 코드 변경을 해봐!

// TODO: 위에서 설명한 대로 struct를 수정해봐!
struct ReportCard {
    grade: f32,
    student_name: String,
    student_age: u8,
}

// TODO: 위에서 설명한 대로 impl 블록을 수정해봐!
impl ReportCard {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}

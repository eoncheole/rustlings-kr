// 축구 경기의 점수 목록(한 줄에 하나씩)이 주어져. 각 줄은
// "<팀1_이름>,<팀2_이름>,<팀1_골>,<팀2_골>" 형식이야.
// 예시: "England,France,4,2" (England가 4골, France가 2골을 넣음).
//
// 팀 이름, 팀이 넣은 총 골 수, 팀이 허용한 총 실점 수를 담고 있는 점수
// 테이블을 만들어야 해.

use std::collections::HashMap;

// 팀의 골 세부 정보를 저장하는 구조체(struct)야.
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // 팀 이름이 키(key)이고, 연관된 구조체가 값(value)이야.
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // 참고: 아직 에러 처리(error handling)를 다루지 않았기 때문에 `unwrap`을 사용해.
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // TODO: 추출한 세부 정보로 점수 테이블을 채워봐.
        // 팀 1이 넣은 골은 팀 2의 실점 수가 된다는 걸 기억해. 마찬가지로,
        // 팀 2가 넣은 골은 팀 1의 실점 수가 돼.
    }

    scores
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}

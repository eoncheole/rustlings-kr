// 컬렉션(collection) 안의 요소들에 대해 작업을 수행할 때, 반복자(Iterator)는
// 필수적이야. 이 모듈은 반복자를 사용하는 구조에 익숙해지고, 반복 가능한
// 컬렉션 안의 요소들을 순회하는 방법을 알려줄 거야.

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterators() {
        let my_fav_fruits = ["banana", "custard apple", "avocado", "peach", "raspberry"];

        // TODO: 배열에 대한 반복자를 만들어봐.
        let mut fav_fruits_iterator = todo!();

        assert_eq!(fav_fruits_iterator.next(), Some(&"banana"));
        assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: `todo!()`를 교체해봐
        assert_eq!(fav_fruits_iterator.next(), Some(&"avocado"));
        assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: `todo!()`를 교체해봐
        assert_eq!(fav_fruits_iterator.next(), Some(&"raspberry"));
        assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: `todo!()`를 교체해봐
    }
}

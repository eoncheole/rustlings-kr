// 구조체는 데이터를 담지만, 로직도 가질 수 있어. 이 연습문제에서는
// `Package` 구조체를 정의하고, 여기에 붙은 로직을 테스트해볼 거야.

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Self {
        if weight_in_grams < 10 {
            // Rust에서 에러를 처리하는 올바른 방법은 아니지만,
            // 에러 처리는 나중에 배울 거야.
            panic!("Can't ship a package with weight below 10 grams");
        }

        Self {
            sender_country,
            recipient_country,
            weight_in_grams,
        }
    }

    // TODO: 함수 시그니처에 올바른 반환 타입을 추가해봐!
    fn is_international(&self) {
        // TODO: 이 메서드를 사용하는 테스트를 읽고 패키지가
        // 언제 국제 배송으로 간주되는지 알아내봐!
    }

    // TODO: 함수 시그니처에 올바른 반환 타입을 추가해봐!
    fn get_fees(&self, cents_per_gram: u32) {
        // TODO: 패키지의 배송비를 계산해봐!
    }
}

fn main() {
    // 여기서 자유롭게 실험해봐.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}

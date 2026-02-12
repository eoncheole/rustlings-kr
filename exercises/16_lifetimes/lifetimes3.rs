// struct가 참조(reference)를 가지고 있을 때도 수명(Lifetime)이 필요해.

// TODO: struct 관련 컴파일러 에러를 고쳐봐.
struct Book {
    author: &str,
    title: &str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}

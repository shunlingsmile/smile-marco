use smile_marco::Builder;

#[derive(Builder)]
struct Book {
    title: String,
    price: i32,
    author: String,
}

fn main() {
    let book = Book::builder()
        .title("Rust Programming".to_string())
        .price(100)
        .author("rust".to_string())
        .build();
    assert_eq!(book.title, "Rust Programming".to_string());
    assert_eq!(book.price, 100);
}
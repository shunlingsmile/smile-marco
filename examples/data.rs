use smile_marco::data;

#[data]
struct Book {
    title: String,
    #[name(cost)] //rename
    price: i32,
    #[exclude] // Do not generate get methods for author
    author: String,
}

fn main() {
    let book = Book::builder()
        .title("Rust Programming".to_string())
        .price(100)
        .author("rust".to_string())
        .build();
    assert_eq!(book.get_title(), &"Rust Programming".to_string());
    assert_eq!(book.get_cost(), &100);
}
use smile_marco::Setter;

#[derive(Setter)]
struct Book {
    title: String,
    #[name(cost)] // rename
    price: i32,
    #[exclude] // Do not generate set methods for author
    author: String,
}

fn main() {
    let mut book = Book {
        title: "Rust Programming".to_string(),
        price: 100,
        author: "rust".to_string(),
    };

    book.set_cost(200);
    assert_eq!(book.price, 200);
}

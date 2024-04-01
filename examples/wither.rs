use smile_marco::{ Wither};

#[derive(Wither)]
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

    let book = book.with_cost(|v| v + 100);
    assert_eq!(book.price, 200);
}
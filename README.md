# smile-marco provides macros such as Getter, Setter, Wither, Builder, etc
These macros generate methods such as get, set, with, and builder for properties within a struct, providing quick access and manipulation. Convenient for daily development.
```toml
[dependencies]
smile-marco = { version = "1.0.0", features = ["full"] }
#or
smile-marco = { version = "1.0.0"}
```
If needed, use partial macros.

```toml
[dependencies]
smile-marco = { version = "1.0.0", features = ["builder","getter","setter","wither"] }
```
# Example
```rust
use smile_marco::{Builder, Getter, Setter, Wither};

#[derive(Builder, Getter, Setter, Wither)]
struct Book {
    title: String,
    #[name(cost)] // rename
    price: i32,
    #[exclude] // Do not generate set methods for author
    author: String,
}

fn main() {
    let mut book = Book::builder()
        .title("Rust Programming".to_string())
        .price(100)
        .author("rust".to_string())
        .build();
    assert_eq!(book.title, "Rust Programming".to_string());
    book.set_title("Rust Book".into());
    assert_eq!(book.title, "Rust Book".to_string());
    assert_eq!(book.get_cost(), &100);
    let mut book = book.with_cost(|c| c + 100);
    assert_eq!(book.price, 200);
}

```
If you need more usage examples, please refer to [examples](./examples)。

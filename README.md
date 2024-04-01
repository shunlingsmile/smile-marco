# smile-marco provides macros such as Getter, Setter, Wither, Builder, etc
These macros generate methods such as get, set, with, and builder for properties within a struct, providing quick access and manipulation. Convenient for daily development.
```toml
[dependencies]
smile-marco = { version = "0.1.0", features = ["full"] }
#or
smile-marco = { version = "0.1.0"}
```
If needed, use partial macros.

```toml
[dependencies]
smile-marco = { version = "0.1.0", features = ["getter","setter"] }
```
# Example
```rust
use smile_marco::Getter;

#[derive(Getter)]
struct Book {
    title: String,
    #[name(cost)] //rename
    price: i32,
    #[exclude] // Do not generate get methods for author
    author: String,
}

fn main() {
    let book = Book {
        title: "Rust Programming".to_string(),
        price: 100,
        author: "rust".to_string(),
    };
    assert_eq!(book.get_title(), &"Rust Programming".to_string());
    assert_eq!(book.get_cost(), &100);
}
```
If you need more usage examples, please refer to [examples](./examples)。

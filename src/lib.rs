//! #   smile-marco provides macros such as `Getter`, `Setter`, `Wither`, `Builder`, etc
//! These macros generate methods such as `get`, `set`, `with`, and `builder` for properties within a struct, providing quick access and manipulation.
//! Convenient for daily development.
//! ```toml
//! [dependencies]
//! smile-marco = { version = "1.0.0", features = ["full"] }
//! #or
//! smile-marco = { version = "1.0.0"}
//! ```
//! If needed, use partial macros.
//! ```toml
//! [dependencies]
//! smile-marco = { version = "1.0.0", features = ["getter","setter"] }
//! ```
//! # Example
//! ```rust
//! use smile_marco::Getter;
//! #[derive(Getter)]
//! struct Book {
//!     title: String,
//!     #[name(cost)] //rename
//!     price: i32,
//!     #[exclude] // Do not generate get methods for author
//!     author: String,
//! }
//!
//!
//!  let book = Book {
//!  title: "Rust Programming".to_string(),
//!  price: 100,
//!  author: "rust".to_string(),
//!  };
//!  assert_eq!(book.get_title(), &"Rust Programming".to_string());
//!  assert_eq!(book.get_cost(), &100);
//! }
//! ```
#[cfg(feature = "builder")]
mod builder;
#[cfg(feature = "full")]
mod data;
#[cfg(feature = "getter")]
mod getter;
#[cfg(feature = "setter")]
mod setter;
mod util;
#[cfg(feature = "wither")]
mod wither;

type TokenStream1 = proc_macro::TokenStream;
type TokenStream2 = proc_macro2::TokenStream;

/// The Getter macro is used to quickly generate the get method
///
/// ```
/// use smile_marco::Getter;
/// #[derive(Getter)]
/// struct Book {
///     title: String,
///     #[name(cost)] //rename
///     price: i32,
///     #[exclude] // Do not generate get methods for author
///     author: String,
/// }
///
/// // may be used as such
/// let book = Book {
///     title: "Rust Programming".to_string(),
///     price: 100,
///     author: "rust".to_string(),
///  };
///  assert_eq!(book.get_title(), &"Rust Programming".to_string());
///  assert_eq!(book.get_cost(), &100);
///
///
/// //Getter Macros automatically generate the following code
/// impl Book {
///     #[inline]
///     pub fn get_title(&self) -> &String {
///         &self.title
///     }
///     #[inline]
///    pub fn get_cost(&self) -> &i32 {
///         &self.price
///     }
/// }
/// ```
///
#[cfg(feature = "getter")]
#[proc_macro_derive(Getter, attributes(exclude, name))]
pub fn getter_derive(input: TokenStream1) -> TokenStream1 {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    getter::gen_ast(&input)
}

/// Setter macros are used to quickly generate set methods
///
/// ```
/// use smile_marco::Setter;
/// #[derive(Setter)]
/// struct Book {
///     title: String,
///     #[name(cost)] // rename
///     price: i32,
///     #[exclude] // Do not generate set methods for author
///     author: String,
/// }
/// // And then you can use it like this.
/// let mut book = Book {
///     title: "Rust Programming".to_string(),
///     price: 100,
///     author: "rust".to_string(),
///  };
///
///  book.set_cost(200);
///  assert_eq!(book.get_cost(), &200);
///
/// //Setter Macros automatically generate the following code
/// impl Book {
///     #[inline]
///     pub fn set_title(&mut self, title: String) {
///         self.title = title;
///     }
///     #[inline]
///     pub fn set_cost(&mut self, cost: i32) {
///         self.price = cost;
///     }
/// }
///
/// ```

#[cfg(feature = "setter")]
#[proc_macro_derive(Setter, attributes(exclude, name))]
pub fn setter_derive(input: TokenStream1) -> TokenStream1 {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    setter::gen_ast(&input)
}

/// Wither macros are used to quickly generate with methods
///
/// ```
/// use smile_marco::Wither;
/// #[derive(Wither)]
/// struct Book {
///     title: String,
///     #[name(cost)] //rename
///     price: i32,
///     #[exclude] // Do not generate with methods for author
///     author: String,
/// }
/// // may be used as such
/// let mut book = Book {
///     title: "Rust Programming".to_string(),
///     price: 100,
///     author: "rust".to_string(),
///  };
///
///  let book = book.with_cost(|v| v+100);
///  assert_eq!(book.price, 200);
///
/// //Wither Macros automatically generate the following code
/// impl Book {
///     #[inline]
///     pub fn with_title<F>(mut self, func: F) -> Self
///         where
///             F: FnOnce(String) -> String,
///     {
///         self.title = func(self.title);
///         self
///     }
///     #[inline]
///     pub fn with_cost<F>(mut self, func: F) -> Self
///         where
///             F: FnOnce(i32) -> i32,
///     {
///         self.price = func(self.price);
///         self
///     }
/// }
///

/// ```
#[cfg(feature = "wither")]
#[proc_macro_derive(Wither, attributes(exclude, name))]
pub fn wither_derive(input: TokenStream1) -> TokenStream1 {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    wither::gen_ast(&input)
}

/// Builder macro
///
/// ```
/// use smile_marco::Builder;
/// #[derive(Builder)]
/// struct Book {
///     title: String,
///     price: i32,
///     author: String,
/// }
///
/// // may be used as such
/// let book = Book::builder()
/// .title("Rust Programming".to_string())
/// .price(100)
/// .author("rust".to_string())
/// .build();
/// assert_eq!(book.title, "Rust Programming".to_string());
/// assert_eq!(book.price, 100);
/// //Builder Macros automatically generate the following code
/// struct BookBuilder {
///     title: Option<String>,
///     price: Option<i32>,
///     author: Option<String>,
/// }
/// impl BookBuilder {
///     #[inline]
///     pub fn new() -> Self {
///         Self {
///             title: None,
///             price: None,
///             author: None,
///         }
///     }
///     #[inline]
///     pub fn build(self) -> Book {
///         Book {
///             title: self.title.expect("title field is not set in Book struct"),
///             price: self.price.expect("price field is not set in Book struct"),
///             author: self.author.expect("author field is not set in Book struct"),
///         }
///     }
///     #[inline]
///     pub fn title(mut self, v: String) -> Self {
///        self.title = Some(v);
///         self
///    }
///     #[inline]
///     pub fn price(mut self, v: i32) -> Self {
///         self.price = Some(v);
///         self
///     }
///     #[inline]
///     pub fn author(mut self, v: String) -> Self {
///         self.author = Some(v);
///         self
///     }
/// }
/// impl Book {
///    #[inline]
///     pub fn builder() -> BookBuilder {
///         BookBuilder::new()
///     }
/// }
///

/// ```
#[cfg(feature = "builder")]
#[proc_macro_derive(Builder)]
pub fn builder_derive(input: TokenStream1) -> TokenStream1 {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    builder::gen_ast(&input)
}

/// data macro
/// ```
/// use smile_marco::data;
/// #[data]
/// struct Book {
///     title: String,
///     price: i32,
///     author: String,
/// }
/// // data macro automatically adds to structure `Getter,Setter,Wither,Builder` macros
/// #[derive(Getter,Setter,Wither,Builder)]
/// struct Book {
///     title: String,
///     price: i32,
///     author: String,
/// }
/// // You can also use the link attribute to specify the source of macros such as Getter、Setter
/// // and use the exclude attribute to exclude macros
/// #[data(link="xxx",exclude = ["Getter", "Setter"])]
/// ```
#[cfg(feature = "full")]
#[proc_macro_attribute]
pub fn data(attr: TokenStream1, item: TokenStream1) -> TokenStream1 {
    data::ast_gen(attr, item)
}

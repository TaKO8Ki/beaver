//! [![github]](https://github.com/TaKO8Ki/beaver)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?labelColor=555555&logo=github
//!
//! This crate is a library for setting up Rust objects inspired by [factory_bot](https://github.com/thoughtbot/factory_bot).
//!
//! ## Dependencies
//! ```toml
//! [dependencies]
//! beaver = "0.1.0"
//! serde = { version = "1.0", features = ["derive"] }
//! ```
//! If you want to use [chrono](https://!docs.rs/chrono/) for your struct fields, `Cargo.toml` would look like this.
//! ```toml
//! [dependencies]
//! beaver = "0.1.0"
//! serde = { version = "1.0", features = ["derive"] }
//! # you need `serde` feature.
//! chrono = { version = "0.4", features = ["serde"] }
//! ```
//!
//! ## Examples
//! You can define your factory like the following.
//! ```rust
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize)]
//! pub struct Post {
//!     id: u16,
//!     title: String,
//!     approved: bool,
//! }
//!
//! beaver::define! {
//!     PostFactory (Post) {
//!         // n is a sequence number.
//!         id -> |n| n,
//!         title -> |n| format!("{}", n),
//!         approved -> |_| false,
//!     }
//! }
//! ```
//!
//! You can use these functions in factory definition.
//! - [sequence](factory/fn.sequence.html): If you want to use a sequence number, you can use this function.
//! - [sequence_a](factory/fn.sequence_a.html): If you want to use a sequence letter, you can use this function.
//!
//! You can use your factory like the following.
//! ```rust
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize, Debug)]
//! pub struct Post {
//!     id: u16,
//!     title: String,
//!     approved: bool,
//! }
//!
//! beaver::define! {
//!     PostFactory (Post) {
//!         id -> |n| n,
//!         title -> |n| format!("{}", n),
//!         approved -> |_| false,
//!     }
//! }
//!
//! fn main() {
//!     use PostFactory;
//!
//!     let post_factory = PostFactory::new();
//!     let post1 = post_factory.build(|_| {});
//!     let post2 = post_factory.build(|_| {});
//!     // overriding attributes of a factory
//!     let post3 = post_factory.build(|post| {
//!         post.id = 1024;
//!         post.title = "foo bar".to_string()
//!     });
//!     println!("{:?}\n{:?}\n{:?}", post1, post2, post3);
//! }
//! ```

mod factory;
mod macros;
mod variable;

pub use factory::Factory;
pub use factory::{new, sequence, sequence_a};

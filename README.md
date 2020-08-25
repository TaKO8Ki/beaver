# beaver

A library for setting up Rust objects inspired by factory_bot.

## Dependencies

```toml
[dependencies]
beaver = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Usage

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u16,
    name: String,
    email: String,
}

fn main() {
    let mut factory = beaver::new(User {
        id: 1,
        name: "beaver".to_string(),
        email: "foo@example.com".to_string(),
    });
    factory.sequence(|factory, n| {
        factory.id = n;
        factory.name = format!("user-{}", n);
        factory.email = format!("user-{}@example.com", n)
    });
    let user1 = factory.create();
    let user2 = factory.create();
    println!("{:?}\n{:?}", user1, user2,);
}
```

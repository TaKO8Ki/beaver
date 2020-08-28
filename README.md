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
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

// Post needs both of Serialize and Deserialize
#[derive(Serialize, Deserialize, Debug)]
struct Post {
    id: u16,
    title: String,
    approved: bool,
    created_at: NaiveDateTime,
}

impl Default for Post {
    fn default() -> Self {
        Post {
            id: 1,
            title: "beaver".to_string(),
            approved: true,
            created_at: NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0),
        }
    }
}

fn main() {
    let post_factory = beaver::new(Post::default(), |post, n| {
        post.id = n;
        post.title = format!("post-{}", n);
        post.created_at = NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0)
    });

    let post1 = post_factory.build();
    let post2 = post_factory.build();
    println!("{:?}\n{:?}", post1, post2);

    let posts = post_factory.build_list(3);
    for post in posts {
        println!("{:?}", post);
    }
}
```

Output:

```sh
Post { id: 1, title: "post-1", approved: true, created_at: 2020-01-01T00:00:00 }
Post { id: 2, title: "post-2", approved: true, created_at: 2020-01-01T00:00:00 }
Post { id: 3, title: "post-3", approved: true, created_at: 2020-01-01T00:00:00 }
Post { id: 4, title: "post-4", approved: true, created_at: 2020-01-01T00:00:00 }
Post { id: 5, title: "post-5", approved: true, created_at: 2020-01-01T00:00:00 }
```

If you use `chrono`, `Cargo.toml` would look like this. 

```toml
[dependencies]
beaver = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
# you need `serde` feature.
chrono = { version = "0.4", features = ["serde"] }
```

## License

Licensed under MIT license ([LICENSE](LICENSE)).

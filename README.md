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
struct Post {
    id: u16,
    title: String,
    approved: bool,
}

impl Default for Post {
    fn default() -> Self {
        Post {
            id: 1,
            title: "beaver".to_string(),
            approved: true,
        }
    }
}

fn main() {
    let post_factory = beaver::new(Post::default(), |ctx| {
        ctx.sequence(1, |post, n| {
            post.id = n;
            post.title = format!("post-{}", n);
        });

        ctx.attribute(|post| post.approved = false);
    });

    let post1 = post_factory.create();
    let post2 = post_factory.create();
    println!("{:?}\n{:?}", post1, post2);

    let posts = post_factory.create_list(3);
    for post in posts {
        println!("{:?}", post);
    }
}
```

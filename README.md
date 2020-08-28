# beaver

A library for setting up Rust objects inspired by factory_bot.

## Dependencies

```toml
[dependencies]
beaver = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
```

## Usage

### [Simple factory](examples/simple.rs)

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

If you want to use `chrono`, `Cargo.toml` would look like this. 

```toml
[dependencies]
beaver = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
# you need `serde` feature.
chrono = { version = "0.4", features = ["serde"] }
```

### [Sub factory vector](examples/sub_factory_vector.rs)

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    id: u16,
    title: String,
    approved: bool,
    tags: Vec<Tag>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tag {
    id: u16,
    name: String,
}

impl Default for Post {
    fn default() -> Self {
        Post {
            id: 1,
            title: "post".to_string(),
            approved: true,
            tags: vec![],
        }
    }
}

impl Default for Tag {
    fn default() -> Self {
        Tag {
            id: 1,
            name: "tag".to_string(),
        }
    }
}

fn main() {
    let tag_factory = beaver::new(Tag::default(), |tag, n| {
        tag.id = n;
        tag.name = format!("tag-{}", n)
    });

    let post_factory = beaver::new(Post::default(), |post, n| {
        post.id = n;
        post.title = format!("post-{}", n);
        // use build_list(number)
        post.tags = tag_factory.build_list(3)
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

Ootput:

```sh
Post { id: 1, title: "post-1", approved: true, tags: [Tag { id: 1, name: "tag-1" }, Tag { id: 2, name: "tag-2" }, Tag { id: 3, name: "tag-3" }] }
Post { id: 2, title: "post-2", approved: true, tags: [Tag { id: 4, name: "tag-4" }, Tag { id: 5, name: "tag-5" }, Tag { id: 6, name: "tag-6" }] }
Post { id: 3, title: "post-3", approved: true, tags: [Tag { id: 7, name: "tag-7" }, Tag { id: 8, name: "tag-8" }, Tag { id: 9, name: "tag-9" }] }
Post { id: 4, title: "post-4", approved: true, tags: [Tag { id: 10, name: "tag-10" }, Tag { id: 11, name: "tag-11" }, Tag { id: 12, name: "tag-12" }] }
Post { id: 5, title: "post-5", approved: true, tags: [Tag { id: 13, name: "tag-13" }, Tag { id: 14, name: "tag-14" }, Tag { id: 15, name: "tag-15" }] }
```

Other examples.

- [sub factory](examples/sub_factory.rs)

## License

Licensed under MIT license ([LICENSE](LICENSE)).

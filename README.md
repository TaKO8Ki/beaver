# beaver

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/TaKO8Ki/beaver/CI/master)](https://github.com/TaKO8Ki/beaver/actions)

beaver is a library for setting up Rust objects inspired by [factory_bot](https://github.com/thoughtbot/factory_bot).

## Dependencies

```toml
[dependencies]
beaver = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
```

If you want to use [chrono](https://docs.rs/chrono/) for your struct fields, `Cargo.toml` would look like this. 

```toml
[dependencies]
beaver = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
# you need `serde` feature.
chrono = { version = "0.4", features = ["serde"] }
```

## Usage

- [Simple Factory](#simple-factory)
- [Sub factory vector](#sub-factory-vector)
- [Others](#others)

### [Simple factory](examples/simple.rs)

```rust
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// Post needs both of Serialize and Deserialize
#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    id: u16,
    title: String,
    approved: bool,
    created_at: NaiveDateTime,
}

mod factory {
    use crate::Post;
    use chrono::NaiveDate;

    // factory definition
    beaver::define! {
        PostFactory (Post) {
            id -> |n| n,
            title -> |n| format!("{}", n),
            approved -> |_| false,
            created_at -> |_| NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0),
        }
    }
}

fn main() {
    use factory::PostFactory;

    let post_factory = PostFactory::new();
    let post1 = post_factory.build(|_| {});
    let post2 = post_factory.build(|_| {});
    // overriding attributes of a factory
    let post3 = post_factory.build(|post| {
        post.id = 1024;
        post.title = "foo bar".to_string()
    });
    println!("{:?}\n{:?}\n{:?}", post1, post2, post3);
}
```

Output:

```sh
Post { id: 1, title: "post-1", approved: true, created_at: 2020-01-01T00:00:00 }
Post { id: 2, title: "post-2", approved: true, created_at: 2020-01-01T00:00:00 }
Post { id: 1024, title: "foo bar", approved: true, created_at: 2020-01-01T00:00:00 }
```

### [Sub factory vector](examples/sub_factory_vector.rs)

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    id: u16,
    title: String,
    approved: bool,
    tags: Vec<Tag>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    id: u16,
    name: String,
}

mod factory {
    use crate::Post;
    use crate::Tag;

    beaver::define! {
        PostFactory (Post) {
            id -> |n| n,
            title -> |n| format!("post-{}", n),
            approved -> |_| true,
            // use `build_list`
            tags -> |n| TagFactory::build_list(3, n),
        }
    }

    beaver::define! {
        TagFactory (Tag) {
            id -> |n| beaver::sequence(100, n),
            name -> |n| format!("tag-{}", n),
        }
    }
}

fn main() {
    use factory::PostFactory;

    let post_factory = PostFactory::new();
    let post1 = post_factory.build(|_| {});
    let post2 = post_factory.build(|_| {});
    println!("{:?}\n{:?}", post1, post2);

    let posts = post_factory.build_list(3, |_| {});
    for post in posts {
        println!("{:?}", post);
    }
}
```

Output:

```sh
Post { id: 1, title: "post-1", approved: true, tags: [Tag { id: 1, name: "tag-1" }, Tag { id: 2, name: "tag-2" }, Tag { id: 3, name: "tag-3" }] }
Post { id: 2, title: "post-2", approved: true, tags: [Tag { id: 4, name: "tag-4" }, Tag { id: 5, name: "tag-5" }, Tag { id: 6, name: "tag-6" }] }
Post { id: 3, title: "post-3", approved: true, tags: [Tag { id: 7, name: "tag-7" }, Tag { id: 8, name: "tag-8" }, Tag { id: 9, name: "tag-9" }] }
Post { id: 4, title: "post-4", approved: true, tags: [Tag { id: 10, name: "tag-10" }, Tag { id: 11, name: "tag-11" }, Tag { id: 12, name: "tag-12" }] }
Post { id: 5, title: "post-5", approved: true, tags: [Tag { id: 13, name: "tag-13" }, Tag { id: 14, name: "tag-14" }, Tag { id: 15, name: "tag-15" }] }
```

### Others

- [sub factory](examples/sub_factory.rs)

## License

Licensed under MIT license ([LICENSE](LICENSE)).

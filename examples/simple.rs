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

// factory definition
beaver::define! {
    PostFactory (Post) {
        id -> |n| n,
        title -> |n| format!("{}", n),
        approved -> |_| false,
        created_at -> |_| NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0),
    }
}

fn main() {
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

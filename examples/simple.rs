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

    // overriding attributes of a factory
    let post1 = post_factory.build(|_| {});
    let post2 = post_factory.build(|_| {});
    let post3 = post_factory.build(|post| {
        post.id = 1024;
        post.title = "foo bar".to_string()
    });
    println!("{:?}\n{:?}\n{:?}", post1, post2, post3);
}

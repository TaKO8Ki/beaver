use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

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

    let post1 = post_factory.build(|post| {
        post.title = "Foo Bar".to_string();
        post.id = 1
    });
    let post2 = post_factory.build(|_| {});
    println!("{:?}\n{:?}", post1, post2);
}

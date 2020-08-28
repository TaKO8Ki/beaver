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
    let post_factory = beaver::nnew(Post::default(), |post, n| {
        post.id = n;
        post.title = format!("post-{}", n)
    });

    let post1 = post_factory.build();
    let post2 = post_factory.build();
    println!("{:?}\n{:?}", post1, post2);

    let posts = post_factory.build_list(3);
    for post in posts {
        println!("{:?}", post);
    }
}

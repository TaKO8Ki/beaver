use chrono::{NaiveDate, NaiveDateTime};
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
    use super::Post;
    use chrono::NaiveDate;

    beaver::define! {
        Post => (
            Post {
                id: 1,
                title: "beaver".to_string(),
                approved: true,
                created_at: NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0),
            }
        ) {
            id -> 2,
            title -> "beaver".to_string(),
            created_at -> NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0),
        }
    }

    beaver::def! {
        use super::Post;
        use chrono::NaiveDate;

        post_facoty => (
            Post {
                id: 1,
                title: "beaver".to_string(),
                approved: true,
                created_at: NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0),
            }
        ) {
            id -> 2,
            title -> "beaver".to_string(),
            created_at -> NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0),
        }
    }
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

use factory::post_facoty::*;

fn main() {
    // let post_factory = beaver::new(Post::default(), |post, n| {
    //     post.id = n;
    //     post.title = format!("post-{}", n);
    //     post.created_at = NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0)
    // });

    // overriding attributes of a factory
    // let mut post1 = post_factory.build(|_| {});
    // let post2 = post_factory.build(|_| {});
    // let post3 = post_factory.build(|post| {
    //     post.id = 1024;
    //     post.title = "foo bar".to_string()
    // });

    // let hoge = factory::f();
    // hoge(&mut post1);

    // println!("{:?}\n{:?}\n{:?}", post1, post2, post3);

    let post_factory = post_facoty::new();
    let post1 = post_factory.build(|_| {});
    println!("{:?}", post1);
}

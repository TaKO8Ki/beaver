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
    beaver::define! {
        use crate::Post;
        use chrono::NaiveDate;

        PostFactory (Post) {
            id -> |n| n,
            title -> |n| format!("{}", n),
            approved -> |_| false,
            created_at -> |_| NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0),
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

use factory::PostFactory;

fn main() {
    let post_factory = PostFactory::new();
    let post1 = post_factory.build(|_| {});
    let post2 = post_factory.build(|_| {});
    println!("{:?}\n{:?}", post1, post2);
}

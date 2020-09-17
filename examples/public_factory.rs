use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// `Post` needs to be public.
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

    beaver::define! {
        // `PostFactory` needs to be public.
        pub PostFactory (Post) {
            id -> |n| n,
            title -> |n| format!("post-{}", n),
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
    println!("{:?}\n{:?}", post1, post2);
}

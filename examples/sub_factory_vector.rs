use crate::factory::PostFactory;
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

mod factory {
    beaver::define! {
        use crate::Post;

        PostFactory (Post) {
            id -> |n| n,
            title -> |n| format!("user-{}", n),
            approved -> |_| false,
            tags -> |n| TagFactory::build_list(3, n),
        }
    }

    beaver::define! {
        use crate::Tag;

        TagFactory (Tag) {
            id -> |n| beaver::sequence(100, n),
            name -> |n| format!("tag-{}", n),
        }
    }
}

fn main() {
    let post_factory = PostFactory::new();
    let posts = post_factory.build_list(3, |_| {});
    for post in posts {
        println!("{:?}", post);
    }
}

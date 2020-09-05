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

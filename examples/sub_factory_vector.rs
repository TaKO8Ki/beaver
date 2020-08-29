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

impl Default for Post {
    fn default() -> Self {
        Post {
            id: 1,
            title: "post".to_string(),
            approved: true,
            tags: vec![],
        }
    }
}

impl Default for Tag {
    fn default() -> Self {
        Tag {
            id: 1,
            name: "tag".to_string(),
        }
    }
}

fn main() {
    let tag_factory = beaver::new(Tag::default(), |tag, n| {
        tag.id = n;
        tag.name = format!("tag-{}", n)
    });

    let post_factory = beaver::new(Post::default(), |post, n| {
        post.id = n;
        post.title = format!("post-{}", n);
        // use build_list(number)
        post.tags = tag_factory.build_list(3, |_| {})
    });

    let post1 = post_factory.build(|_| {});
    let post2 = post_factory.build(|_| {});
    println!("{:?}\n{:?}", post1, post2);

    let posts = post_factory.build_list(3, |_| {});
    for post in posts {
        println!("{:?}", post);
    }
}

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    id: u16,
    title: String,
    approved: bool,
    file: File,
    tags: Vec<Tag>,
    created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    id: u16,
    path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    id: u16,
    name: String,
}

impl PartialEq for Post {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.title == other.title
            && self.approved == other.approved
            && self.file == other.file
            && self.tags == other.tags
            && self.created_at == other.created_at
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.path == other.path
    }
}

mod factory {
    use super::{File, Post, Tag};
    use chrono::NaiveDate;
    // factory definition
    beaver::define! {
        PostFactory (Post) {
            id -> |n| n,
            title -> |n| format!("post-{}", n),
            approved -> |_| false,
            file -> |n| FileFactory::build(n),
            tags -> |n| TagFactory::build_list(3, n),
            created_at -> |_| NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0),
        }
    }

    beaver::define! {
        FileFactory (File) {
            id -> |n| n,
            path -> |n| format!("path/to/file-{}", n),
        }
    }

    beaver::define! {
        TagFactory (Tag) {
            id -> |n| n,
            name -> |n| format!("tag-{}", n),
        }
    }
}

#[test]
fn is_builds_struct() {
    use factory::{FileFactory, PostFactory};

    let file_factory = FileFactory::new();
    let post_factory = PostFactory::new();

    let post1 = post_factory.build(|_| {});
    let post2 = post_factory.build(|_| {});
    let post3 = post_factory.build(|post| {
        post.id = 1024;
        post.title = "foo".to_string();
        post.file = file_factory.build(|_| {})
    });
    let posts = post_factory.build_list(2, |_| {});

    assert_eq!(
        post1,
        Post {
            id: 1,
            title: "post-1".to_string(),
            approved: false,
            file: File {
                id: 1,
                path: "path/to/file-1".to_string()
            },
            tags: vec![
                Tag {
                    id: 1,
                    name: "tag-1".to_string()
                },
                Tag {
                    id: 2,
                    name: "tag-2".to_string()
                },
                Tag {
                    id: 3,
                    name: "tag-3".to_string()
                }
            ],
            created_at: NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0)
        }
    );
    assert_eq!(
        post2,
        Post {
            id: 2,
            title: "post-2".to_string(),
            approved: false,
            file: File {
                id: 2,
                path: "path/to/file-2".to_string()
            },
            tags: vec![
                Tag {
                    id: 4,
                    name: "tag-4".to_string()
                },
                Tag {
                    id: 5,
                    name: "tag-5".to_string()
                },
                Tag {
                    id: 6,
                    name: "tag-6".to_string()
                }
            ],
            created_at: NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0)
        }
    );
    assert_eq!(
        post3,
        Post {
            id: 1024,
            title: "foo".to_string(),
            approved: false,
            file: File {
                id: 1,
                path: "path/to/file-1".to_string()
            },
            tags: vec![
                Tag {
                    id: 7,
                    name: "tag-7".to_string()
                },
                Tag {
                    id: 8,
                    name: "tag-8".to_string()
                },
                Tag {
                    id: 9,
                    name: "tag-9".to_string()
                }
            ],
            created_at: NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0)
        }
    );
    assert_eq!(
        posts,
        vec![
            Post {
                id: 4,
                title: "post-4".to_string(),
                approved: false,
                file: File {
                    id: 4,
                    path: "path/to/file-4".to_string()
                },
                tags: vec![
                    Tag {
                        id: 10,
                        name: "tag-10".to_string()
                    },
                    Tag {
                        id: 11,
                        name: "tag-11".to_string()
                    },
                    Tag {
                        id: 12,
                        name: "tag-12".to_string()
                    }
                ],
                created_at: NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0)
            },
            Post {
                id: 5,
                title: "post-5".to_string(),
                approved: false,
                file: File {
                    id: 5,
                    path: "path/to/file-5".to_string()
                },
                tags: vec![
                    Tag {
                        id: 13,
                        name: "tag-13".to_string()
                    },
                    Tag {
                        id: 14,
                        name: "tag-14".to_string()
                    },
                    Tag {
                        id: 15,
                        name: "tag-15".to_string()
                    }
                ],
                created_at: NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0)
            }
        ]
    )
}

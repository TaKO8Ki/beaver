use crate::variable;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::marker::PhantomData;

pub struct Factory<'a, T>
where
    T: Serialize + Deserialize<'a>,
{
    pub model: String,
    pub sequence: Cell<u16>,
    pub gen_func: Box<dyn Fn(&mut T, u16)>,
    pub _maker: PhantomData<&'a T>,
}

/// Creates [Factory](struct.Factory.html) for the `model`.
pub fn new<'a, T>(model: T, suite: Box<dyn Fn(&mut T, u16)>) -> Factory<'a, T>
where
    T: Serialize + Deserialize<'a>,
{
    Factory {
        model: serde_json::to_string(&model).unwrap(),
        sequence: Cell::new(1),
        gen_func: suite,
        _maker: PhantomData,
    }
}

/// Returns a consecutive term. The first term is `from` argument.
///
/// # Usage
/// ```rust
/// use chrono::{NaiveDate, NaiveDateTime};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize)]
/// pub struct Post {
///     id: u16,
///     title: String,
///     approved: bool,
///     created_at: NaiveDateTime,
/// }
///
/// beaver::define! {
///     PostFactory (Post) {
///         id -> |n| n,
///         // First post's title is "post-100". Second post's title is "post-101".
///         title -> |n| format!("post-{}", beaver::sequence(100, n)),
///         approved -> |_| false,
///         created_at -> |_| NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0),
///     }
/// }
/// ```
pub fn sequence(from: u16, n: u16) -> u16 {
    from + n - 1
}

/// Returns a consecutive letter. The first letter is `from` argument.
///
/// # Example
/// - When `from` is "z" and `n` is 1, this function returns "z".
/// - When `from` is "z" and `n` is 2, this function returns "aa".
/// - When `from` is "z" and `n` is 20, this function returns "ba".
///
/// # Usage
/// ```rust
/// use chrono::{NaiveDate, NaiveDateTime};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize)]
/// pub struct Post {
///     id: u16,
///     title: String,
///     approved: bool,
///     created_at: NaiveDateTime,
/// }
///
/// beaver::define! {
///     PostFactory (Post) {
///         id -> |n| n,
///         // First post's title is "post-a". Second post's title is "post-b".
///         title -> |n| format!("post-{}", beaver::sequence_a("a", n)),
///         approved -> |_| false,
///         created_at -> |_| NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0),
///     }
/// }
/// ```
pub fn sequence_a(from: &str, n: u16) -> String {
    to_alphabet(*variable::ALPHABET_INDEX.get(from).unwrap() as u128 + (n - 1) as u128)
}

/// Converts a number to Excel like base 26.
///
/// # Example
/// - When n is 0, this function returns "a".
/// - When n is 25, this function returns "z".
/// - When n is 26, this function returns "aa".
fn to_alphabet(n: u128) -> String {
    if n > 25 {
        format!(
            "{}{}",
            to_alphabet((n - n % 26) / 26 - 1),
            variable::ALPHABET[(n % 26) as usize]
        )
    } else {
        variable::ALPHABET[(n % 26) as usize].to_string()
    }
}

impl<'a, T> Factory<'a, T>
where
    T: Serialize + Deserialize<'a>,
{
    /// Builds a struct from [Factory](struct.Factory.html).
    pub fn build<O>(&'a self, f: O) -> T
    where
        O: Fn(&mut T),
    {
        let mut model = serde_json::from_str(self.model.as_str()).unwrap();
        let suite = &self.gen_func;
        suite(&mut model, self.sequence.get());
        f(&mut model);
        self.sequence.set(self.sequence.get() + 1);
        model
    }

    #[doc(hidden)]
    pub fn build_n<O>(&'a self, n: u16, f: O) -> T
    where
        O: Fn(&mut T),
    {
        let mut model = serde_json::from_str(self.model.as_str()).unwrap();
        let suite = &self.gen_func;
        suite(&mut model, n);
        f(&mut model);
        self.sequence.set(self.sequence.get() + 1);
        model
    }

    /// Builds a vector of struct from [Factory](struct.Factory.html).
    pub fn build_list<O>(&'a self, number: u16, f: O) -> Vec<T>
    where
        O: Fn(&mut T),
    {
        let mut list = vec![];
        for _ in 0..number {
            list.push(self.build(&f))
        }
        list
    }

    #[doc(hidden)]
    pub fn build_list_n<O>(&'a self, number: u16, n: u16, f: O) -> Vec<T>
    where
        O: Fn(&mut T),
    {
        let mut list = vec![];
        for i in number * (n - 1) + 1..number * (n - 1) + number + 1 {
            list.push(self.build_n(i, &f))
        }
        list
    }
}

#[cfg(test)]
mod tests {
    use crate::factory::{new, sequence, sequence_a, to_alphabet};
    use chrono::{NaiveDate, NaiveDateTime};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_new() {
        #[derive(Serialize, Deserialize)]
        struct Post {
            id: u16,
            title: String,
            approved: bool,
            file: File,
            tags: Vec<Tag>,
            created_at: NaiveDateTime,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct File {
            id: u16,
            path: String,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct Tag {
            id: u16,
            name: String,
        }

        impl Default for File {
            fn default() -> Self {
                File {
                    id: 1,
                    path: "path/to/beaver.png".to_string(),
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

        impl Default for Post {
            fn default() -> Self {
                Post {
                    id: 1,
                    title: "post".to_string(),
                    approved: true,
                    file: File::default(),
                    tags: vec![],
                    created_at: NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0),
                }
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

        let post_factory = new(
            Post::default(),
            Box::new(|post, n| {
                post.id = n;
                post.title = format!("post-{}", n);
                post.approved = false;
                post.file = File {
                    id: n,
                    path: format!("path/to/file-{}", n),
                };
                post.tags = vec![
                    Tag {
                        id: n,
                        name: format!("tag-{}", n),
                    },
                    Tag {
                        id: n + 1,
                        name: format!("tag-{}", n + 1),
                    },
                    Tag {
                        id: n + 2,
                        name: format!("tag-{}", n + 2),
                    },
                ];
                post.created_at = NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0)
            }),
        );

        assert_eq!(
            post_factory.model,
            r#"{"id":1,"title":"post","approved":true,"file":{"id":1,"path":"path/to/beaver.png"},"tags":[],"created_at":"2020-01-01T00:00:00"}"#
        );
        assert_eq!(post_factory.sequence.get(), 1);

        let mut post = Post::default();
        let f = post_factory.gen_func;
        f(&mut post, 1);

        assert_eq!(post.id, 1);
        assert_eq!(post.title, "post-1");
        assert_eq!(post.approved, false);
        assert_eq!(
            post.file,
            File {
                id: 1,
                path: "path/to/file-1".to_string()
            }
        );
        assert_eq!(
            post.tags,
            vec![
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
            ]
        );
        assert_eq!(
            post.created_at,
            NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn test_sequence() {
        assert_eq!(sequence(2, 1), 2);
        assert_eq!(sequence(2, 2), 3);
        assert_eq!(sequence(2, 3), 4);
    }

    #[test]
    fn test_sequence_a() {
        assert_eq!(sequence_a("a", 1), "a");
        assert_eq!(sequence_a("a", 2), "b");
        assert_eq!(sequence_a("a", 3), "c");

        assert_eq!(sequence_a("b", 1), "b");
        assert_eq!(sequence_a("b", 2), "c");
        assert_eq!(sequence_a("b", 3), "d");

        assert_eq!(sequence_a("z", 1), "z");
        assert_eq!(sequence_a("z", 2), "aa");
        assert_eq!(sequence_a("z", 3), "ab");

        assert_eq!(sequence_a("z", 28), "ba");
        assert_eq!(sequence_a("z", 677), "zz");
        assert_eq!(sequence_a("z", 678), "aaa");
        assert_eq!(sequence_a("z", 975), "all");
        assert_eq!(sequence_a("z", 9975), "ntp");
    }

    #[test]
    fn test_to_alphabet() {
        assert_eq!(to_alphabet(0), "a");
        assert_eq!(to_alphabet(25), "z");
        assert_eq!(to_alphabet(26), "aa");
        assert_eq!(to_alphabet(52), "ba");
        assert_eq!(to_alphabet(701), "zz");
        assert_eq!(to_alphabet(702), "aaa")
    }
}

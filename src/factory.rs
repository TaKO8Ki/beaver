use crate::variable;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::marker::PhantomData;

pub struct Factory<'a, F, T>
where
    T: Serialize + Deserialize<'a>,
    F: Fn(&mut T, u16),
{
    pub model: String,
    pub sequence: Cell<u16>,
    pub gen_func: F,
    _maker: PhantomData<&'a T>,
}

pub fn new<'a, T, S>(model: T, suite: S) -> Factory<'a, S, T>
where
    T: Serialize + Deserialize<'a>,
    S: Fn(&mut T, u16),
{
    Factory {
        model: serde_json::to_string(&model).unwrap(),
        sequence: Cell::new(1),
        gen_func: suite,
        _maker: PhantomData,
    }
}

pub fn sequence(from: u16, n: u16) -> u16 {
    from + n - 1
}

pub fn sequence_a(from: &str, n: u16) -> String {
    to_alphabet(*variable::ALPHABET_INDEX.get(from).unwrap() as u128 + (n - 1) as u128)
}

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

impl<'a, F, T> Factory<'a, F, T>
where
    T: Serialize + Deserialize<'a>,
    F: Fn(&mut T, u16),
{
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
}

#[cfg(test)]
mod tests {
    use crate::factory::{new, sequence, sequence_a};
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
            created_at: NaiveDateTime,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct File {
            id: u16,
            path: String,
        }

        impl Default for File {
            fn default() -> Self {
                File {
                    id: 1,
                    path: "path/to/beaver.png".to_string(),
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
                    created_at: NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0),
                }
            }
        }

        impl PartialEq for File {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id && self.path == other.path
            }
        }

        let file_factory = new(File::default(), |file, n| {
            file.id = n;
            file.path = format!("path/to/file-{}", n)
        });

        let post_factory = new(Post::default(), |post, n| {
            post.id = n;
            post.title = format!("post-{}", n);
            post.approved = false;
            post.file = file_factory.build(|_| {});
            post.created_at = NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0)
        });

        assert_eq!(
            post_factory.model,
            r#"{"id":1,"title":"post","approved":true,"file":{"id":1,"path":"path/to/beaver.png"},"created_at":"2020-01-01T00:00:00"}"#
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
}

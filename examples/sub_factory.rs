use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct File {
    id: u16,
    path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u16,
    name: String,
    file: File,
}

impl Default for File {
    fn default() -> Self {
        File {
            id: 1,
            path: "path/to/beaver.png".to_string(),
        }
    }
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 1,
            name: "beaver".to_string(),
            file: File::default(),
        }
    }
}

fn main() {
    let file_factory = beaver::new(File::default(), |file, n| {
        file.id = n;
        file.path = format!("path/to/file-{}", n)
    });

    let user_factory = beaver::new(User::default(), |user, n| {
        user.id = beaver::sequence(1000, n);
        user.name = format!("user-{}", beaver::sequence_a("x", n));
        user.file = file_factory.build(|_| {})
    });

    let users = user_factory.build_list(10, |_| {});
    for user in users {
        println!("{:?}", user)
    }
}

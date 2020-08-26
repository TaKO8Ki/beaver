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
    let user_factory = beaver::new(User::default(), |ctx| {
        let file_factory = beaver::new(File::default(), |ctx| {
            ctx.sequence(1, |file, n| {
                file.id = n;
                file.path = format!("path/to/file-{}.png", n);
            });
        });

        ctx.sequence(100, |user, n| {
            user.id = n;
            user.name = format!("user-{}", n);
        });

        ctx.sub_factory(Box::new(move |user| user.file = file_factory.build()));
    });

    let users = user_factory.build_list(10);
    for user in users {
        println!("{:?}", user)
    }
}

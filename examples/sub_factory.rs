use factory::UserFactory;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    id: u16,
    path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
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

mod factory {
    beaver::define! {
        use crate::User;

        UserFactory (User) {
            id -> |n| n,
            name -> |n| format!("user-{}", n),
            file -> |n| FileFactory::build(n),
        }
    }

    beaver::define! {
        use crate::File;

        FileFactory (File) {
            id -> |n| n,
            path -> |n| format!("path/to/file-{}", n),
        }
    }
}

fn main() {
    let user_factory = UserFactory::new();
    let users = user_factory.build_list(10, |_| {});
    for user in users {
        println!("{:?}", user);
    }
}

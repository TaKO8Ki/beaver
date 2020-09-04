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

mod factory {
    use crate::File;
    use crate::User;

    beaver::define! {
        UserFactory (User) {
            id -> |n| n,
            name -> |n| format!("user-{}", n),
            file -> |n| FileFactory::build(n),
        }
    }

    beaver::define! {
        FileFactory (File) {
            id -> |n| n,
            path -> |n| format!("path/to/file-{}", n),
        }
    }
}

fn main() {
    use factory::UserFactory;

    let user_factory = UserFactory::new();
    let users = user_factory.build_list(10, |_| {});
    for user in users {
        println!("{:?}", user);
    }
}

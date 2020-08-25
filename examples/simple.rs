use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u16,
    name: String,
    email: String,
}

fn main() {
    let mut factory = beaver::new(User {
        id: 1,
        name: "beaver".to_string(),
        email: "example@example.com".to_string(),
    });
    factory.sequence(|factory, n| {
        factory.id = n;
        factory.name = format!("user-{}", n);
        factory.email = format!("user-{}@example.com", n)
    });
    let user1 = factory.create();
    let user2 = factory.create();
    let user3 = factory.create();
    let user4 = factory.create();
    let user5 = factory.create();
    println!(
        "{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
        user1, user2, user3, user4, user5
    );
    let users = factory.create_list(3);
    for user in users {
        println!("{:?}", user);
    }
}

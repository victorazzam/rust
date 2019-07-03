fn main() {
    let mut user1 = User {
        active: true,
        sign_in_count: 1,
        username: String::from("user1"),
        email: String::from("user1@example.com")
    };
    println!("Email: {}", user1.email);
    user1.email = String::from("me@user1.com"); // or using .to_string()
    println!("New email: {}", user1.email);

    let user2 = gen("user2".to_string());
    println!("{} {} {} {}",
             user2.username,
             user2.email,
             user2.active,
             user2.sign_in_count
            );

    let user3 = User {
        username: "user3".to_string(),
        email: "user3@protonmail.ch".to_string(),
        ..user2
    };
    println!("{} {} {} {}",
             user3.username,
             user3.email,
             user3.active,
             user3.sign_in_count
            );
}

fn gen(username: String) -> User {
    let email = String::from(format!("{}@example.com", username));
    User {
        username,
        email,
        sign_in_count: 1,
        active: true
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

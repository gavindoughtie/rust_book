fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("Hello, {}!", user1.email);
    let mut user2 = build_user("user2@example.com".to_string(), "Fred".to_string());
    user2.email = "user2@gmail.com".to_string();
    println!("Hello {}", user2.label("/"));
    let user3 = User {
        email: String::from("gavin@example.com"),
        ..user2
    };
    println!("user3: {}/{}/{:#?}", user3.email, user3.username, user3);
    let user4 = User::build("user4@example.com".to_string(), "user4username".to_string());
    println!("user4: {}", user4.label(":"));
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn label(&self, sep: &str) -> String {
        format!("{}{}{}", self.username, sep, self.email)
    }
}

impl User {
    fn build(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

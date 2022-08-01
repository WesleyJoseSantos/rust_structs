struct User {
    email: String,
    username: String,
    sign_in_count: u64,
    active: bool
}

fn handle_struct() {
    let mut user1 = User {
        email: String::from("wesley@google.com"),
        username: String::from("wesley1234"),
        sign_in_count: 10,
        active: true
    };

    let name = user1.username;
    user1.username = String::from("newUser");

    let user2 = build_user(
        String::from("banana.com"), 
        String::from("banana")
    );

    let user3 = User {
        email: String::from("aquimnest@google.com"), 
        username: String::from("aquiminest"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
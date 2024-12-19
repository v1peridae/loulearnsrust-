// Define struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Fields can only be changed if the instance is mutable
    user1.email = String::from("newemail@example.com");

    // Create a new user with some values that are in user1
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 //Copies the remaining fields from user1
    };

    // A tuple struct
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);


    println!("User 1 email: {}", user1.email);
    println!("User 2 username: {}", user2.username);
    println!("Black color RGB: ({}, {}, {})", black.0, black.1, black.2);
}

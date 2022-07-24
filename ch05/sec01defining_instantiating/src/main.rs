fn main() {
    let user1 = User {
        email: String::from("user1@mail.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 0,
    };
    // user1.active = false; // the instant should be mutable to do this
    // Rust doesn't allow us to mark only certain fields as muable
    let mut user2 = User {
        email: String::from("user2@mail.com"),
        username: String::from("user2"),
        ..user1 // the remaining fields as same as user1
    };
    user2.active = false;
    // debuging for programming output, use {:?} (or {:#?} for pretty print) to print
    println!("user2: {:?}", user2);

    // tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

#[derive(Debug)]
struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

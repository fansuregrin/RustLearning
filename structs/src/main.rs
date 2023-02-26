fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Ada Lovelace"),
        email: String::from("ada@gmail.com"),
        sign_in_count: 1
    };
    user1.active = false;
    if user1.active {
        println!("{}\t{}\t{}", user1.username, user1.email, user1.sign_in_count);
    }

    let user2 = build_user(String::from("charles@outlook.com"), String::from("Charles Babbage"));
    println!("{}\t{}\t{}", user2.username, user2.email, user2.sign_in_count);

    let user3 = User {
        active: true,
        ..user1     // The syntax .. specifies that the remaining fields not explicitly set
                    // should have the same value as the fields in the given instance.
    };
    println!("{}\t{}\t{}", user3.username, user3.email, user3.sign_in_count);
    // println!("{}\t{}\t{}", user1.username, user1.email, user1.sign_in_count);  //ownership moved

    // tuple structs
    let red = Color(255, 0, 0);
    let origin = Point(0, 0, 0);
    println!("#{:02x}{:02x}{:02x}", red.0, red.1, red.2);
    println!("({}, {}, {})", origin.0, origin.1, origin.2);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,   // field init shorthand syntax
        email,      // field init shorthand syntax
        sign_in_count: 1
    }
}

struct Color(u8, u8, u8);
struct Point(i32, i32, i32);

// unit-like structs
#[allow(dead_code)]
struct AlwaysEqual;

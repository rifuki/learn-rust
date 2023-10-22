struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i32
}

struct Color(i32, i32, i32);
struct Point(i32, i32);
struct MyStruct; // unit like struct

impl MyStruct {
    fn new() {
        println!("MyStruct Called");
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("admin@gmail.com"),
        username: String::from("admin"),
        active: true,
        sign_in_count: 1
    };
    user1.email = String::from("administrator@gmail.com");
    println!("user1 username: \"{}\"", user1.username);
    println!("user1 email: \"{}\"", user1.email);

    let user2 = build_user(String::from("user2@gmail.com"), String::from("user2"));
    println!("user2 username: \"{}\"", user2.username);
    // unit like struct
    println!("user2 email: \"{}\"", user2.email);

    let user3 = User {
        email: String::from("user3@gmail.com"),
        username: user1.username.clone(),
        active: user1.active,
        sign_in_count: user1.sign_in_count
    };
    println!("user3 username: \"{}\"", user3.username);
    println!("user3 email: \"{}\"", user3.email);

    let user4 = User {
        email: String::from("user4@gmail.com"),
        ..user1
    };
    println!("user4 username: \"{}\"", user4.username);
    println!("user4 email: \"{}\"", user4.email);

    let black = Color(0, 0, 0);
    println!("Black (red: {}, green: {}, blue: {});", black.0, black.1, black.2);
    let center = Point(-5, 5);
    let Point(x, y) = center;
    println!("Center (x: {}, y: {});", x, y);

    
}

fn build_user(email: String, username: String) -> User {
    User {
        email, username,
        active: true,
        sign_in_count: 1
    }
}

fn main() {

    { // s is not valid here, it's not yet declared
        let s: &str = "hello";
        // do with stuff s
        print!("s: {}", s);
    } // this scope is over, s is no longer valid

    let mut s = String::from("hello"); // s is a string literal
    s.push_str(" world");
    println!("s {}", s);

    let x = 1048;
    let y = x; // copy <= Copy Trait (i32, i32)
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let s2 = &s1;
    // let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    {
        let s = String::from("hello"); // s goes into scope

        takes_ownership(s); // s value moves into the function
                            // s is no longer valid
        let x = 5; // x goes into scope
        makes_copy(x); // x value is  copy
        // use x afterward
        println!("x: {}", x);
    } // x goes out of scope, then s
    
    {
        let s1 = gives_ownership();
        println!("s1: {}", s1);
        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2);
        println!("s3: {}", s3);
    }

    {
        let s1 = String::from("hello");
        let (s2, length) = calculate_length(s1);
        println!("s2: {}, has length: {}", s2, length);
    }
    
}

fn takes_ownership(s: String) {
    println!("some: {}", s);
} // here, some string goes out scope, drop is called

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_str = String::from("world");
    some_str
}

fn takes_and_gives_back(kotoba: String) -> String {
    kotoba
}

fn calculate_length(kotoba: String) -> (String, usize) {
    let length = kotoba.len();
    (kotoba, length)
}

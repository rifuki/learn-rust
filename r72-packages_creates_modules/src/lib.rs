mod front_of_house;

pub use front_of_house::hosting;

pub fn eat_at_restaurant() {
    println!("eat at restaurant");
    hosting::add_to_waitlist();
}

// use std::collections::HashMap;
//
// use std::fmt;
// use std::io;
//
// fn func1() -> fmt::Result {
//     Ok(())
// }
// fn func2() -> io::Result<()> {
//     Ok(())
// }
// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//
//     let mut map: HashMap<i32, i32> = HashMap::new();
// }

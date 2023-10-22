#![allow(unused)]
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

#[derive(Debug)]
enum IpAddrNew {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u32, u32, u32)
}

impl Message {
    fn call(&self) {}
}

struct QuitMessage; // unit #[derive(Debug)]

struct MoveMessage { // plain struct
    x: i32,
    y: i32
}

struct WriteMessage(String); // tuple struct
struct ChageColorMessage(u32, u32, u32); // tuple struct

fn main() {
    {
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        let home = IpAddr { kind: IpAddrKind::V4, address: String::from("127.0.0.1") };

        println!("four: {:#?}", four);
        println!("six: {:#?}", six);
    }
    {
        let home4 = IpAddrNew::V4(127, 0, 0, 1);
        let home6 = IpAddrNew::V6(String::from("::1"));
        println!("{:?}", home4);
        println!("{:?}", home6);
    }
    {
        let msg = Message::Write("hello".to_string());
    }
    {
        let some_number = Some(5);
        let some_str = Some("hello");
        let abset_number: Option<i32> = None;
        
        let x = 5;
        let y = Some(5);
        let sum = x + y.unwrap();
        println!("sum: {}", sum);
    }
}

fn main() {
    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("The length of \"{}\" is {}", s1, len);
    }

    {
        let mut s = String::from("hello");
        change(&mut s); // create a mutable reference
        println!("s is \"{}\"", s);
    }

    {
        let a = String::from("hello");
        let r1 = &a;
        let r2 = &a;
        println!("{}, {}", r1, r2);
    }

    {
        let mut b = String::from("hello");
        let x1 = &b;
        let x2 = &b;
        println!("{}, {}", x1, x2);
        let x3 = &mut b;
        println!("{}", x3);
    } 

    {
        // let reference = dangle(); 
    }

    {
        let sample = String::from("hello world");
        let word = first_word(&sample);
        println!("first word: {}", word);
    }
    {
        let sample2 = "konichiwa world";
        let word = first_word(&sample2);
        println!("first word: {}", word);
    }
}

fn calculate_length(kotoba: &String) -> usize { // s is a reference to a string
    kotoba.len()
}

fn change(some_kotoba: &mut String) {
    some_kotoba.push_str(" world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // s is dropped
//
//

fn first_word(kotoba: &str) -> &str {
    let bytes = kotoba.as_bytes();
    
    for (i, item) in bytes.iter().enumerate() {
        if item == &32 {
            return &kotoba[..i];
        }
    }
    
    kotoba
}

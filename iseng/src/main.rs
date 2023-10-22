fn main() {
    let my_kotoba = String::from(" katakan");
    if let Some(first_word) = first_word(&my_kotoba) {
        println!("first_word = {}", first_word);
    }
}

fn first_word(kotoba: &String) -> Option<&str> {
    let word_bytes = kotoba.as_bytes();

    if word_bytes.len() <= 0 {
        return None;
    }
    
    for (i, word) in word_bytes.iter().enumerate() {
        if word == &32 {
            return Some(&kotoba[..i]);
        }
    }
    Some(kotoba)
}

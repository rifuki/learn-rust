mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        mod back_of_house {
            fn fix_incorrect_order() {
                cook_order();
                super::serve_order();
            }
            fn cook_order() {} 
        }
        fn take_payment() {}
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn eat(toast: &str) -> Self {
            Self { toast: String::from(toast), seasonal_fruit: String::from("apple") }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad
    }
}

use self::front_of_house::hosting;

pub fn eat_at_resturant() {
    // absolute path
    hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = self::front_of_house::Breakfast::eat("sari roti");
    meal.toast = String::from("borobudur");
    meal.seasonal_fruit = String::from("bakpia");
    println!("meal.toast: {}", meal.toast);

    let order1 = front_of_house::Appetizer::Salad;
}

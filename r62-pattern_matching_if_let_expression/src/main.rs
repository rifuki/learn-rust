#![allow(unused)]
#[derive(Debug)]
enum Asia {
    Indonesia,
    Malaysia,
    Singapura
}

#[derive(Debug)]
enum Coin {
    Head,
    Flag(Asia)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Head => 1,
        Coin::Flag(state) => {
            println!("Country from: {:?}", state);
            5
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    {
        let coin_indonesia: u8 = value_in_cents(Coin::Flag(Asia::Indonesia));
        println!("coin indonesia: {:?}", coin_indonesia);
    }

    {
        let five = Some(5);
        let six = plus_one(five);
        println!("six: {}", six.unwrap());
        let none_0 = None;
        let nonon = plus_one(none_0);
        if let None = nonon {
            println!("the value is empty");
        }
    }

    {
        let dice_roll: u8 = 9;
        match dice_roll {
            2 => go_to_jail(),
            3 => buy_property(),
            7 => buy_ticket(),
            _ => reroll()
        }

        fn buy_property() { println!("Buy Property"); }
        fn buy_ticket() { println!("Buy Ticket"); }
        fn go_to_jail() { println!("Go to Jail"); }
        fn reroll() { println!("Reroll"); }
    }

    {
        let config_max = Some(3u8);
        match config_max {
            Some(max) => println!("The maximum is: {}", max),
            _ => ()
        }

        if let Some(max) = config_max {
            println!("The maximum is: {}", max);
        }
    }

    {
        let coin = Coin::Head;
        let mut count = 0u8;
        if let Coin::Flag(location) = coin {
            println!("Location from: {:?}", location)
        } else {
            count += 1;
        }
    }
}

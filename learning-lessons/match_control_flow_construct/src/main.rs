// simple match with coins
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents2(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// there we check if the coint2 mathces some state, in our case the some UsState enum
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn check_quarrer() {
    value_in_cents(Coin2::Quarter(UsState::Alaska));
}


fn main() {
    println!("Hello, world!");
}

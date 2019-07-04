fn main() {
    let coin1 = Coin::Penny;
    let coin2 = Coin::Dime;
    let coin3 = Coin::Quarter(USState::Maine);
    println!("Cents in coin1: {}", get_value(coin1));
    println!("Cents in coin2: {}", get_value(coin2));
    println!("Cents in coin3: {}", get_value(coin3));
}

fn get_value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => {
            println!("Lucky dime!");
            10
        },
        Coin::Quarter(state) => {
            println!("Got a quarter from {:?}!", state);
            25
        },
    }
}

#[derive(Debug)]
enum USState {
    Alaska,
    California,
    Maine,
    NewYork,
    Wyoming,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

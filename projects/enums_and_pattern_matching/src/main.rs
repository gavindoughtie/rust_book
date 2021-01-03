#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)] // so we can inspect the state in a minute
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    route(&home);
    route(&loopback);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x
        + (match y {
            None => 0,
            Some(val) => val,
        });
    let alt_sum = x + if let Some(val) = y { val } else { 0 };
    let coins = [
        Coin::Dime,
        Coin::Quarter(UsState::Alaska),
        Coin::Quarter(UsState::Alabama),
        Coin::Penny,
        Coin::Nickel,
    ];
    for coin in coins.iter() {
        println!("coin: {:#?}, value: {}", coin, value_in_cents(&coin));
    }

    println!("sum: {}, alt_sum: {}", sum, alt_sum);
}

fn route(addr: &IpAddr) {
    println!("v4 address: {:#?}", addr);
}

fn _route(addr: &IpAddr) {
    println!("addr: {:#?}", addr);
}

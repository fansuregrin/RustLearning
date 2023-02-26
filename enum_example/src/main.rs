#[allow(unused)]
fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    route(home);
    route(loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    let five = Some(5);
    let rocket = Some('🚀');
    let absent_number: Option<i32> = None;
    let six = plus_one(five);
    let none = plus_one(absent_number);

    let coin1 = Coin::Penny;
    let coin2 = Coin::Quarter(UsState::Alabama);
    println!("coin1: {}¢", value_in_cents(coin1));
    println!("coin2: {}¢", value_in_cents(coin2));

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let coin3 = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    if let Coin::Quarter(state) = coin3 {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

#[allow(unused)]
fn route(ip_kind: IpAddrKind) {}

#[allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {  // multiple lines of code in a match arm
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // matches are exhaustive!
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

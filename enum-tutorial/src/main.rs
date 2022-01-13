enum IpAddrKind {
    V4(String),
    V6(String),
}

fn route(ip_type: IpAddrKind) { }

// fn main() {
//     // let four = IpAddrKind::V4;
//     // let six = IpAddrKind::V6;
//     //
//     // route(IpAddrKind::V4);
//
//     let home = IpAddrKind::V4(String::from("127.0.0.1"));
//     let loopback = IpAddrKind::V6(String::from("::1"));
//
//     let x: i8 = 5;
//     let y: Option<i8> = Some(5);
//
//     let sum = x + y;
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // anything
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
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

#[derive(Deebug)]
enum UsState {
    Alabama,
    Alaska,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // let some_u8_value = 0u8;
    // match some_u8_value {
    //     1 => println!("one"),
    //     3 => println!("three"),
    //     5 => println!("five"),
    //     7 => println!("seven"),
    //     _ => (),
    // }

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
}

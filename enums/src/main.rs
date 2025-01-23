use std::net::{Ipv4Addr, Ipv6Addr};
use std::option::Option;

fn main() {
    {
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;
    }

    {
        route(IpAddrKind::V4);
        route(IpAddrKind::V6);
    }

    {
        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
    }

    {
        let home = IpAddr1::V4(127, 0, 0, 1);

        let loopback = IpAddr1::V6(String::from("::1"));
    }

    {
        let m = Message::Write(String::from("hello"));
        m.call();
    }

    {
        let some_number = Some(5);
        let some_char = Some('e');

        // let absent_number: Option<i32> = None;
    }

    {
        let x: i8 = 5;
        let y: i8 = 6;
        //let y: Option<i8> = Some(5);

        let sum = x + y;
    }

    {
        value_in_cents(Coin::Quarter(UsState::Alaska));
    }

    {
        let five = Some(5);
        let six=plus_one(five);
        let none=plus_one(None);
    }

    {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            other => move_player(other),
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn move_player(num_spaces: u8) {}
    }

    {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => rerolled(),
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn rerolled() {}
    }

    {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => (), // do nothing
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}

    }

    {
        let config_max= Some(3u8);
        match config_max {
            Some(max) => println!("max is {}", max),
            _ => (),
        }
    }

    {
        let config_max = Some(3u8);
        if let Some(max) = config_max {
            println!("max is {}", max);
        }
    }

    {
        let coin = Coin::Quarter(UsState::Alaska);
        let mut count = 0;
        match coin {
            Coin::Quarter(state) => println!("Quarter state: {:?}", state),
            _ => count += 1,
        }
    }

    {
        let coin = Coin::Quarter(UsState::Alaska);
        let mut count = 0;
        if let Coin::Quarter(state) = coin {
            println!("Quarter state: {:?}", state);
        } else {
            count += 1;
        }
    }
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn route(ip_kind: IpAddrKind) {}

enum IpAddr1 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum IpAddr2 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn call(&self) {}
}

enum Option1<T> {
    None,
    Some(T),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }


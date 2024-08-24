// enumの列挙子は直接値を紐づけることができる
// それぞれの型が違ってもいい
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn call(&self) {
        println!("{}", "enumはメソッドを持つことができます")
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// パターンマッチング
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let v4 = IpAddr::V4(127, 0, 0, 1);
    let v6 = IpAddr::V6(String::from("::1"));
    v4.call();
    v6.call();

    let c1 = Coin::Dime;
    let value = value_in_cents(c1);
    println!("value: {}", value);

    let c2 = Coin::Penny;
    let value = value_in_cents(c2);
    println!("value: {}", value);

    let c3 = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(c3);
    println!("value: {}", value);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = Some(0u8);
    // 3の時だけ何かしたい
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // if let で書き換える
    // if let パターン = 式
    if let Some(3) = some_u8_value {
        println!("three")
    } else {
        println!("not three")
    }
}

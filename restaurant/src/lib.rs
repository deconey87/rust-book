// モジュールを定義
// mod front_of_house {
//     // モジュールの中にモジュールを持てる
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}
//         fn serve_order() {}
//         fn take_payment() {}
//     }
// }

// 別ファイルに切り出したモジュールを使う
// こうすると、同じ名前のファイル名から探してくれる
mod front_of_house;
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 絶対パス
    crate::front_of_house::hosting::add_to_waitlist();
    // 相対パス
    front_of_house::hosting::add_to_waitlist();
    // useを使ってスコープに持ち込む
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheel");
    println!("I'd like {} toast please", meal.toast);
    // seasonal_fruitはpubではないので読み取れない
    // println!("seasonal_fruit:{} ", meal.seasonal_fruit);

    let order1 = back_of_house::Appetizer::Soup;
}

// モジュールツリー
// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // enumはpubにすると全て公開される
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

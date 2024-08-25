use std::fmt::Display;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("largest char is {}", result);

    // 有効なライフタイム
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("the longest string is {}", result);

    // 有効でない可能性がある
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     // string2のライフタイムはこのブロック中しかない
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // resultは、すでに無効になったstring2を参照している可能性があるので、安全ではない
    // println!("the longest string is {}", result);

    let x = String::from("abc");
    let y = String::from("123456");
    let announce = 123;
    longest_with_an_announcement(x.as_str(), y.as_str(), announce);
}

// &[i32] はスライスへの参照
// fn largest(list: &[i32]) -> i32 {
//     let mut largest = list[0];
//     for &number in list.iter() {
//         if number > largest {
//             largest = number
//         }
//     }
//     largest
// }

// ↓ こんな書き方もある
// fn largest<T>(list: &[T]) -> T
// where
//     T: PartialOrd + Copy,
// {

// ジェネリックとトレイトを使って汎用的になっている
// trait boundにより、PartialOrdとCopyを実装している型、に制限している
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        // > で比較するためには、 std::cmp::PartialOrd が実装されていないといけない
        if item > largest {
            largest = item
        }
    }
    largest
}

// xとy、そして関数の帰り値は同じライフタイム（'a）であることを示す
// longestの返す参照は、xとyの参照が有効である間だけ有効になる
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Tは Display を実装する型を受け取る（トレイト境界）
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');

    // let y = 6;
    // ↑ これは文。値を返さないので、このような書き方はできない
    // let x = (let y = 6);

    let y = {
        // ↑この式は、4と評価されるブロック
        let x = 3;
        x + 1 // セミコロンがないことに注目
    };
    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

// 関数のparameterの型は必ず書かないといけない
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// これも正しい関数
fn five() -> i32 {
    5
}

// これも。x + 1にセミコロンをつけないことで、式のままにしないといけない
//
fn plus_one(x: i32) -> i32 {
    x + 1
}

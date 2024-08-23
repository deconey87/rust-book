fn main() {
    let number = 3;

    // 条件式部分に括弧はいらない。条件はboolでないといけない
    if number < 5 {
        println!("conditions was true");
    } else {
        println!("conditions was false");
    }

    let condition = true;
    // if自体も式なので、値を返してくる
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

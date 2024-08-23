fn main() {
    // 変数は基本 immutable
    // mut をつけると mutable になる
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 定数はconst
    // 定数名は大文字と_だけで構成する
    // 型は自分で書かないといけない
    // const MAX_POINTS: u32 = 100_000;

    // シャドーイング
    // 前に定義した変数と同じ名前の変数を定義できる
    let x = 5;
    let x = x + 1;
    {
        // ブロック中だけシャドーイング
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x); // 12
    }
    println!("The value of x is: {}", x); // 6

    // シャドーイングにより型を変えつつ、同じ変数名を使いまわせる
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The length of spaces: {}", spaces);

    // 一方この代入は許されない
    // let spaces = "    ";
    // space = spaces.len();

    // タプル
    let tup = (500, 6.4, 1);
    // 0から始まる添字でアクセスできる
    println!("The value of first is: {}", tup.0);

    // 分解
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    // 配列
    let a = [1, 2, 3, 4, 5];
    // [3,3,3,3,3] と同じ
    let a = [3; 5];
}

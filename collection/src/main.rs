use std::collections::HashMap;

fn main() {
    // -----ベクタ（Vec<T>）---------
    // i32型を格納するベクタの宣言
    let v: Vec<i32> = Vec::new();

    // 初期値を与えて宣言する方法
    // この場合は型推論が効く
    let v = vec![1, 2, 3];

    // 更新
    // この場合も、pushする値から推論される
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // 読み取り
    let v = vec![1, 2, 3, 4, 5];
    // 参照を使う方法
    let third = &v[2];
    // NOTE: printlnが&i32をデリファレンスして、欲しい値を表示してくれる
    println!("The third element is {}", third);

    // getを使う ... Option<&T> を受け取る
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 存在しない要素にアクセスしようとするときこれはpanicする
    // let not_exist = &v[100];

    // こっちはNone
    let not_exist = v.get(100);
    if let None = not_exist {
        println!("not exists")
    }

    // ループ
    let v = vec![100, 32, 57];
    // 参照なので所有権をとらない
    // もし v でforを回すとその後vが使えなくなる
    for i in &v {
        println!("{}", i);
    }
    println!("The length of v is {}", v.len());

    // 値を変更するループ
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    // ---------文字列-------------
    // str ... Rustのcoreとしての型。 文字列スライス。　&strでよく使われる
    // String ... 標準ライブラリで提供される、要素が追加できて、可変で、所有権のある文字列

    // 文字列生成
    // 空文字列
    let mut s = String::new();

    // 文字列リテラルからの変換
    let data = "initial string";
    let s = data.to_string();

    // 初期値あり
    let s = String::from("initial");

    // 文字列の更新
    // 1. データのpush
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
    s.push('!');
    println!("{}", s);

    // 2. + を使う
    // fn add(self, s: &str) -> String が内部的に使われるので、s1は所有権が奪われる
    // &s2は、コンパイラによって&s2[..]に変換される （deref coercion）
    let s1 = String::from("hello, ");
    println!("{}", s1);
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    // println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);

    // 3. format! マクロ
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    // --------- ハッシュマップ -----------

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 取得
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(score) => println!("score is {}", score),
        None => println!("not found."),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 更新
    // 1. 上書き
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // 2. キーがなければ挿入
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // 3. 古い値に基づいて更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

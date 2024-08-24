fn main() {
    // 文字列リテラル.不変であり、コンパイル時に内容も大きさも決まっているので、バイナリに直接ハードコードされる
    // コンパイル時にサイズが不明（入力を受け付けるなど）場合は使えない
    let s = "Hello";
    println!("{}", s);

    // String型
    // 可変かつ伸長可能なテキストを保持するために、コンパイル時には不明な量のメモリをヒープに確保して内容を保持する
    // メモリを解放するのは、strがスコープを抜けた時
    {
        let mut str1 = String::from("Hello");
        str1.push_str(", world!");
        println!("{}", str1);
    }
    // ここでは strは解放されているので呼び出せない
    // println!("{}", str1);

    // ムーブ
    {
        // String型はスタックに 長さと許容量と、ヒープ（実際の文字列が格納されている）へのポインタを持つ
        let s1 = String::from("hello");
        println!("{}", s1);
        // ここで、s2へs1がコピーされ、s1はもう不要と考える（ムーブ）
        let s2 = s1;
        // ↓これはもう呼び出せない
        // println!("{}", s1);
        println!("{}", s2);
    }
    // ↑スコープを抜けるのでs2が解放される

    // クローン
    // 本当にStringのヒープデータ自体をコピーしたいときは cloneする
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("{}", s1);
        println!("{}", s2);
    }

    // スタックのみのデータ
    // 整数などのコンパイル時にサイズが決まってるものは、実際のデータをコピーする
    // 元の変数を無効化しない
    {
        let x = 5;
        let y = x;
        println!("y = {}", y);
        println!("x = {}", x); // xが無効になってない
    }

    // 関数に変数を渡すと、代入のようにコピーやムーブが起きる
    // sが String::from("hello") でできたデータの所有権を持つ
    let s = String::from("hello");
    println!("{} from main", s);
    // 所有権が some_string にうつる
    takes_ownership(s);
    // ここから↓でsは無効
    // println!("{} from main", s);

    let x = 5;
    println!("{} from main", x);
    // i32はただコピーされるだけ
    makes_copy(x);
    // だからここでもxは有効
    println!("{} from main", x);

    let s1 = String::from("hello");
    // let len = calculate_length_only(s1);
    // これは動かない。所有権がcalculate_length_onlyのs1にムーブしてしまうため。
    // println!("The length of {} is {}", s1, len);
    // ↓のように、関数から返せばいいが面倒
    // let (s2, len) = calculate_length(s1);
    // println!("The length of {} is {}", s2, len);

    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);

    // 参照は不変なので、可変な参照を得るには mut
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // --------- スライス ----------
    let s = String::from("hello world");
    let hello = &s[0..5]; // [..5] でもいい
    let world = &s[6..11]; // [6..] でもいい
    println!("{}", hello);
    println!("{}", world);

    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    println!("{}", word);

    // 文字列リテラルはそれ自体が文字列スライス
    let my_string_literal = "hello world";
    let word = first_word(my_string_literal);
    println!("{}", word);

    // このコードはコンパイルエラーにならないが、問題がある
    // let mut s = String::from("hello world");
    // let index = first_word2(&s);
    // println!("{}", s);
    // println!("{}", index);

    // sへの参照はこの時点で存在していないので、sの可変借用ができる
    // これによってsは空文字列になるが.. indexは変更前のもの(5)を保持している
    // indexがもはや意味ある値で無くなっている
    // s.clear();
    // println!("{}", s);
    // println!("{}", index);

    // 一方、このコードはコンパイルエラーになる
    // let mut s = String::from("hello world");

    // 不変な借用がおきる
    // wordは、sの不変参照になる（スライス）
    // let word = first_word(&s[..]);

    // s.clear()は、sの可変借用をするが、word（sの不変参照）が生きているのでエラー
    // s.clear();
    // println!("{}", word);
}

fn takes_ownership(some_string: String) {
    println!("{} in takes_ownership", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{} in makes_copy", some_integer);
}

// fn calculate_length_only(s: String) -> usize {
//     s.len()
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// }

// 所有権ではなく、オブジェクトへの参照を受け取る
// 所有権が移ってこないので関数実行されても呼び出し元の変数は無効にならない
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// &str は文字列スライスを示す型、参照を返す
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    // 文字列全体のスライス
    &s[..]
}

// 切り出されるべき単語の終わりのindexを返す
// usizeを返すので、sの参照はこの関数の実行完了でなくなる
fn first_word2(s: &str) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

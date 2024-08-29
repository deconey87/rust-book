// コンスリスト
// これはコンパイルできない。コンパイルのためには、型のサイズの大きさがわからないといけないが、
// 再帰的な型はどのくらい大きくなるのか、コンパイラは判断できない
// enum List {
//     Cons(i32, List),
//     Nil,
// }

// Box<T>はポインタなので、データのサイズに左右されずに Box<T> に必要な領域が決まる
// だからコンパイルできる
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // スマートポインタ
    // ポインタのように振る舞うだけでなく、追加のメタデータと能力があるデータ構造

    // Bop<T>
    // ヒープにデータを確保し、スタックにはそのポインタを持つ
    let b = Box::new(5);
    // Boxは、データがスタックにあるのと同じような方法でアクセスできる
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // 参照外しの基本
    let x = 5;
    let y = &x; // yは &i32（i32型への参照）

    assert_eq!(5, x);
    assert_eq!(5, *y); // 参照外しして、値を見に行く

    // 参照ではなくBoxを使う
    let z = Box::new(x);
    // 参照外しして、値を見に行く
    assert_eq!(5, *z);

    // これは z = 5 を表示する。
    // Displayトレイトによって、暗黙的に *z になっている
    println!("z = {}", z);

    // 独自実装のスマートポインタ MyBox を使用する
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // ↑ のコードはRustによって暗黙的に以下のように処理される
    // assert_eq!(5, *(y.deref()));

    // deref coercion
    // Derefトレイトを実装している型への参照を、別の型の参照に変換する
    hello("jack"); // 普通の呼び出し &str
    let m = MyBox::new(String::from("Rust"));
    // &m は &MyBox<String>だが、 MyBoxがDerefを実装しているので、 deref() が呼び出されて &String になる
    // そして、StringもDerefが実装されているので &str になって、関数が求める型と一致する
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointer created.");
    // Dropトレイトのdropは、スコープを抜けたら自動で呼ばれ、明示的に呼び出すことはできないが、
    // 早期にDropする必要がある場合、std::mem::dropを呼び出すとdropできる
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

// Boxの独自実装
use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// スマートポインタはDefefを実装する必要がある
// これにより、普通の参照のように扱えるようになる
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("hello, {}", name);
}

// スマートポインタはDropを実装する
// これにより、値がスコープを抜けるときに処理を実行できる
// 例えば、Box<T>はヒープの領域を解放している

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // ここで必要なクリーンアップとかを行う
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}

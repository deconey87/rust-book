// コンスリスト
// これはコンパイルできない。コンパイルのためには、型のサイズの大きさがわからないといけないが、
// 再帰的な型はどのくらい大きくなるのか、コンパイラは判断できない
// enum List {
//     Cons(i32, List),
//     Nil,
// }

// Box<T>はポインタなので、データのサイズに左右されずに Box<T> に必要な領域が決まる
// だからコンパイルできる
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

use std::rc::Rc;
// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

use std::cell::RefCell;
// RcにRefcellを抱えさせることで、複数の所有者を持ちそして、可変化できる値を作る
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
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

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

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

    // Rc<T> 参照カウント

    // このコードはコンパイルできない
    // bリストを作成するときに、aがムーブされるので
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    // Rc<T>を使うと、単独の値に複数の所有者を持つことができる
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("count after creating a = {}", Rc::strong_count(&a));
    // // aへの参照カウントが1増える
    // let b = Cons(3, Rc::clone(&a));
    // println!("count after creating b = {}", Rc::strong_count(&a));
    // {
    //     let c = Cons(4, Rc::clone(&a));
    //     println!("count after creating c = {}", Rc::strong_count(&a));
    // }
    // println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // プリミティブな数値（i32）を直接操作したいので *（デリファレンス）を書く必要がある
    *(value.borrow_mut()) += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
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

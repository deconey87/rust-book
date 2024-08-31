// 独自のBox実装（データはヒープに置かれない）
use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 参照外しのためにはDerefトレイトを実装する必要がある
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    // xへの参照
    let y = &x;
    assert_eq!(5, x);
    // 参照外しでポインタから値を追いかける
    assert_eq!(5, *y);

    // Boxは参照外しができる
    let z = Box::new(x);
    assert_eq!(5, *z);

    let a = MyBox::new(x);
    // Derefトレイトが実装されているので、*で参照外しができる
    assert_eq!(5, *a);
    // Rustによって、以下のように暗黙的に処理されている
    // assert_eq!(5, *(a.deref()));

    // deref coercion
    // 関数に、型定義と合わないものを渡すと、Rustが勝手にderef()を呼び出して参照外しする
    let s = String::from("Rust");
    // &String → &strに変換される
    hello(&s);
    let m = MyBox::new(s);
    // &MyBox<String> → &String → &str と変換される
    hello(&m);
    // deref coercion機能がなかったら、mを参照外ししてStringにし、
    // そこから文字列スライスを作らないといけない
    let deref_self = &(*m)[..];
    hello(deref_self);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

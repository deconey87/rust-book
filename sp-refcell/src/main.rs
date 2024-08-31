// Rc<T> は複数の所有者を持てる不変参照
// RefCell<T> は不変参照でも内部を変更できる
// よって、これを組み合わせることで、複数の所有者を持ち、かつ内部を可変にできる

#[derive(Debug)]
// Rc<RefCell<i32>> ... 複数の所有者を持てる、内部可変な i32
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};

fn main() {
    // 後ほど直接アクセスできるように
    // RCにより複数の所有者ができている
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // valueがRcなのにborrow_mut()が呼べるのは、自動参照外し（auto-dereferencing）が起きているから
    // メソッドを呼び出す時、Rustが自動で、 & / &mut / * をつけてくれる
    *value.borrow_mut() += 10;
    // これと同じ。value.borrow_mut()は、自動で参照外しされている。
    // RefMut<i32>が帰ってくるので、それをさらに *参照外しして、値を変更している
    // *(*value).borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

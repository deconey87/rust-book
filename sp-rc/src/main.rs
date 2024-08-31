enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;
use List::{Cons, Nil};

fn main() {
    // Rc<T> は、値への参照の数を追跡する
    // 参照が0になったら値は片付けられる
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // ここでは自分自身がいるので 1
    println!("count after creating a = {}", Rc::strong_count(&a));

    // ここで、aへの参照が1つ増える（Rc<T>をクローンすると増える）
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        // もう一つ増えて..
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    // cがスコープを抜けたので、この時点では参照カウントは2つ
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

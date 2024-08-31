use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    // Nodeが子ノードに直接アクセスできるように、Rc<Node> として所有権を共有できるようにする
    // また、ノードの子供を変更できるように、RefCell<T>とする
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // 末尾の葉
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        // leafのstrong_count = {}, weak_count = {}
        // strong = 1, weak = 0
        // 自分自身があるから
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        // 枝
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        // 弱い参照で、leafの親にbranchをセットする
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            // branchのstrong_count = {}, weak_count = {}
            // strong = 1, weak = 1
            // 自分自身（strong）、leafの親としてweakで1
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        // strong = 2, weak = 0
        // 自分自身と、branchの子として
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
        // ここでbranchがスコープを外れる
        // weakカウントは関係なく、strongが0になるので解放される
    }

    // leafの親Node（branch）は解放されていなくなった
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // strong = 1, weak = 0
    // leafは単独で存在するようになる
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

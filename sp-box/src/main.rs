// コンスリスト
enum List {
    // 再帰的な構造は、そのままデータを持たせると、コンパイラが必要な領域を計算できない（無限）のでエラーになる
    // Cons(i32, List),
    // Boxはポインタなので、そのサイズはデータの大きさに依らないからOK（usize）
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // Boxの基本
    // Boxはヒープにデータを確保し、スタックにはそのポインタを持つ
    let b = Box::new(5);
    // BoxはDerefトレイトを実装しているため、*bを明示的に書かなくても中身が表示される
    println!("b = {}", b);

    // コンパイル時には内側から評価されていく
    // 1. Box::new(Nil) が評価されて、Nilがヒープ、そのポインタがスタックにできる
    // 2. Box::new(Cons(3, Box::new(Nil))) が評価されて、データがヒープにできる。Box::new(Nil)のポインタもデータの一部なのでヒープに保存され、
    // スタックにあったBox::new(Nil)のポインタは不要なので消える
    // これが繰り返されるので、最終的に最初のポインタはスタックに保存されているが、その後はヒープに保存されたデータを辿っていくことになる
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // わかりやすくバラしたもの
    // スタックには `list1`, `list2`, `list3`, `list4` のポインタがそれぞれ保存される
    let list1 = Box::new(Nil);
    let list2 = Box::new(Cons(3, list1));
    let list3 = Box::new(Cons(2, list2));
    let list4 = Box::new(Cons(1, list3));
}

struct CustomSmartPointer {
    data: String,
}

// スマートポインタはDropトレイトを実装することが多い
// ここで、クリーンアップなどをする
// 例えばBox<T>はヒープのメモリを解放するなどする
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("CustomSmartPointers created.");
    // Dropトレイトを実装していると、スコープを抜ける時に自動でdrop()が呼ばれる
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    // std::mem::drop()で、早期にdropさせることもできる
    drop(c);
    println!("CustomSmartPointers dropped before the end of main.");
}

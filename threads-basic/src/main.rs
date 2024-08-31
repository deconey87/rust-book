use std::thread;
use std::time::Duration;

// Rustの標準ライブラリのスレッド実装は1対1（スレッドを立ち上げると、1つのカーネルスレッドとして処理される）
// メインスレッドの実行が完了すると、立ち上げたスレッドも停止する
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // ここでjoin()を呼び出すと、立ち上げたスレッドが終了するまで以下の処理が実行されないので、
    // spawnedのループが全て完了した後に、メインループが出力される（出力が混ざらない）
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // joinHandleに対してjoin()を呼び出すと、ハンドルが表すスレッドが終了するまで現在実行中のスレッドをブロックされるので、
    // 立ち上げたスレッドが終了するまでメインスレッドも終了しない
    handle.join().unwrap();

    let v = vec![1, 2, 3];
    // 普通のクロージャー呼び出し
    // vの値をキャプチャしていることがわかる
    // let c = || println!("Here's a vecror: {:?}", v);
    // c();

    // スレッドを立ち上げで、そこでクロージャーを呼び出すが、キャプチャできない
    // スレッドがどのくらい生きるかわからないので、vが参照できるかどうかコンパイラがわからない
    // let handle = thread::spawn(|| println!("Here's a vector: {:?}", v));
    // よって、moveにより所有権を取得する必要がある
    let handle = thread::spawn(move || println!("Here's a vector: {:?}", v));
    // もはやvはメインスレッドで使えない（moveしているので）
    // println!("{:?}", v);
    handle.join().unwrap();
}

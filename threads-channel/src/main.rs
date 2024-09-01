use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // txが送信側、rxが受信側
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi!");
        tx.send(val).unwrap();
    });

    // recv() はメインスレッドをブロックして、値が流れてくるまで待つ
    // try_recv() はすぐにResultを返すので、ブロックしないで何かメインスレッドでやりたい時に使う（try_recv()を何回か呼び出す感じ）
    let received = rx.recv().unwrap();
    println!("Got {}", received);

    let (tx2, rx2) = mpsc::channel();
    // 送信元を複数にするためにクローンしている
    let tx2clone = mpsc::Sender::clone(&tx2);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("threads"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2clone.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // イテレーターとして使える
    // recv() しなくても全てのsend() を待つのは。
    // スレッドが終了すると、tx2はドロップされるので、チャンネルが閉じられるから
    for received in rx2 {
        println!("Got: {}", received);
    }
}

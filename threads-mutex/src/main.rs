use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Mutex::new(5);

    {
        // Mutex内部のデータにアクセスするためにlockを獲得する
        // MutexGuardはスマートポインタ
        let mut num = m.lock().unwrap();
        // Derefトレイトによって、参照外しすると内部のi32にアクセスできる
        *num = 6;
        // Dropトレイトによって、lockの解除が自動でされる
    }

    println!("m = {:?}", m);

    // 複数スレッドでMutex<T>を共有する
    // Arc<T> はRc<T>がスレッドセーフになってると思えばいい
    // counterが、クロージャーに所有権を取られないようにするために必要
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()
    }

    println!("Result: {}", *counter.lock().unwrap());
}
